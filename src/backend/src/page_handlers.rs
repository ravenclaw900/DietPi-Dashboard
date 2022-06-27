use anyhow::Context;
use futures::stream::SplitSink;
use futures::SinkExt;
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;
use warp::ws::Message;

use crate::{handle_error, json_msg, shared, systemdata};

type SocketSend = SplitSink<warp::ws::WebSocket, warp::ws::Message>;
type RecvChannel = Receiver<Option<shared::Request>>;

fn main_handler_getter(
    cpu_collector: &mut psutil::cpu::CpuPercentCollector,
    net_collector: &mut psutil::network::NetIoCountersCollector,
    prev_data: &mut shared::NetData,
) -> anyhow::Result<shared::SysData> {
    Ok(shared::SysData {
        cpu: systemdata::cpu(cpu_collector)?,
        ram: systemdata::ram()?,
        swap: systemdata::swap()?,
        disk: systemdata::disk()?,
        network: systemdata::network(net_collector, prev_data)?,
        temp: systemdata::temp(),
    })
}

pub async fn main_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    let mut cpu_collector = handle_error!(
        psutil::cpu::CpuPercentCollector::new().context("Couldn't init cpu collector"),
        return
    );

    let mut net_collector = psutil::network::NetIoCountersCollector::default();
    let mut prev_data = match net_collector.net_io_counters() {
        Ok(counters) => shared::NetData {
            received: counters.bytes_recv(),
            sent: counters.bytes_sent(),
        },
        Err(_) => shared::NetData {
            received: u64::MAX,
            sent: u64::MAX,
        },
    };

    loop {
        tokio::select! {
            biased;
            data = data_recv.recv() => match data {
                Some(Some(_)) => {},
                _ => break,
            },
            res = socket_send
            .send(json_msg!(&handle_error!(main_handler_getter(&mut cpu_collector, &mut net_collector, &mut prev_data), shared::SysData::default()), continue))
            => {
                sleep(Duration::from_secs(1)).await;
                if res.is_err() {
                    break
                }
            },
        }
    }
}

fn process_handler_helper(data: &shared::Request) -> anyhow::Result<()> {
    let process = psutil::process::Process::new(
        data.args[0]
            .parse::<u32>()
            .with_context(|| format!("Invalid pid {}", data.args[0]))?,
    )
    .with_context(|| format!("Couldn't make process from pid {}", data.args[0]))?;
    tracing::info!(
        "{}ing process {}",
        data.cmd.trim_end_matches('e'),
        process.pid()
    );
    match data.cmd.as_str() {
        "terminate" => process.terminate(),
        "kill" => process.kill(),
        "suspend" => process.suspend(),
        "resume" => process.resume(),
        _ => (Ok(())),
    }
    .with_context(|| format!("Couldn't {} process {}", data.cmd, process.pid()))?;
    Ok(())
}

pub async fn process_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    loop {
        tokio::select! {
            biased;
            data = data_recv.recv() => match data {
                Some(Some(data)) => handle_error!(process_handler_helper(&data)),
                _ => break,
            },
            res = socket_send
                .send(json_msg!(
                    &shared::ProcessList {
                        processes: handle_error!(systemdata::processes().await, Vec::new()),
                    }, continue
                )) => {
                    sleep(Duration::from_secs(1)).await;
                    if res.is_err() {
                        break
                    }
                },
        }
    }
}

pub async fn software_handler_helper(
    data: &shared::Request,
) -> anyhow::Result<shared::DPSoftwareList> {
    // We don't just want to run dietpi-software without args
    anyhow::ensure!(!data.args.is_empty(), "Empty dietpi-software args");

    let mut cmd = tokio::process::Command::new("/boot/dietpi/dietpi-software");
    let mut arg_list = vec![data.cmd.as_str()];
    for element in &data.args {
        arg_list.push(element.as_str());
    }
    tracing::info!("{}ing software with ID(s) {:?}", data.cmd, data.args);
    let out = std::string::String::from_utf8(
        cmd.args(arg_list)
            .output()
            .await
            .context("Couldn't get DietPi-Software output")?
            .stdout,
    )
    .context("Invalid DietPi-Software output")?
    .replace('', "");

    let software = systemdata::dpsoftware().await?;
    Ok(shared::DPSoftwareList {
        uninstalled: software.0,
        installed: software.1,
        response: out,
    })
}

pub async fn software_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    let software = handle_error!(systemdata::dpsoftware().await, (Vec::new(), Vec::new()));
    if socket_send
        .send(json_msg!(
            &shared::DPSoftwareList {
                uninstalled: software.0,
                installed: software.1,
                response: String::new(),
            },
            return
        ))
        .await
        .is_err()
    {
        return;
    }
    while let Some(Some(data)) = data_recv.recv().await {
        let out = handle_error!(
            software_handler_helper(&data).await,
            shared::DPSoftwareList::default()
        );
        if socket_send.send(json_msg!(&out, continue)).await.is_err() {
            break;
        }
    }
}

pub async fn management_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    if socket_send
        .send(json_msg!(
            &handle_error!(systemdata::host().await, shared::HostData::default()),
            return
        ))
        .await
        .is_err()
    {
        return;
    }
    while let Some(Some(data)) = data_recv.recv().await {
        // Don't care about the Ok value, so remove it to make the type checker happy
        handle_error!(Command::new(&data.cmd)
            .spawn()
            .map(|_| ())
            .with_context(|| format!("Couldn't spawn command {}", &data.cmd)));
    }
}

pub async fn service_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    if socket_send
        .send(json_msg!(
            &shared::ServiceList {
                services: handle_error!(systemdata::services().await, Vec::new()),
            },
            return
        ))
        .await
        .is_err()
    {
        return;
    }
    while let Some(Some(data)) = data_recv.recv().await {
        handle_error!(Command::new("systemctl")
            .args([&data.cmd, data.args[0].as_str()])
            .spawn()
            .map(|_| ()) // Don't care about the Ok value, so remove it to make the type checker happy
            .with_context(|| format!("Couldn't {} service {}", &data.cmd, &data.args[0])));
        if socket_send
            .send(json_msg!(
                &shared::ServiceList {
                    services: handle_error!(systemdata::services().await, Vec::new()),
                },
                continue
            ))
            .await
            .is_err()
        {
            break;
        }
    }
}

async fn browser_refresh(path: &std::path::Path) -> anyhow::Result<shared::BrowserList> {
    let dir_path = path
        .parent()
        .with_context(|| format!("Couldn't get parent of path {}", path.display()))?;

    Ok(shared::BrowserList {
        contents: systemdata::browser_dir(std::path::Path::new(dir_path)).await?,
    })
}

async fn browser_handler_helper(data: &shared::Request) -> anyhow::Result<shared::BrowserList> {
    use tokio::fs;

    match data.cmd.as_str() {
        "cd" => {
            return Ok(shared::BrowserList {
                contents: systemdata::browser_dir(std::path::Path::new(&data.args[0])).await?,
            });
        }
        "copy" => {
            let mut num = 2;
            while std::path::Path::new(&format!("{} {}", &data.args[0], num)).exists() {
                num += 1;
            }
            fs::copy(&data.args[0], format!("{} {}", &data.args[0], num))
                .await
                .with_context(|| {
                    format!("Couldn't copy file {0} to {0} {1}", &data.args[0], num)
                })?;
        }
        "rm" => {
            fs::remove_file(&data.args[0])
                .await
                .with_context(|| format!("Couldn't delete file at {}", &data.args[0]))?;
        }
        "rmdir" => {
            fs::remove_dir_all(&data.args[0])
                .await
                .with_context(|| format!("Couldn't delete directory at {}", &data.args[0]))?;
        }
        "mkdir" => {
            fs::create_dir(&data.args[0])
                .await
                .with_context(|| format!("Couldn't create directory at {}", &data.args[0]))?;
        }
        "mkfile" => {
            fs::write(&data.args[0], "")
                .await
                .with_context(|| format!("Couldn't create file at {}", &data.args[0]))?;
        }
        "rename" => {
            fs::rename(&data.args[0], &data.args[1])
                .await
                .with_context(|| {
                    format!(
                        "Couldn't rename file {} to {}",
                        &data.args[0], &data.args[1]
                    )
                })?;
        }
        _ => {}
    }

    browser_refresh(std::path::Path::new(&data.args[0])).await
}

pub async fn browser_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    // Get initial listing of $HOME
    if socket_send
        .send(json_msg!(
            &shared::BrowserList {
                contents: handle_error!(
                    systemdata::browser_dir(std::path::Path::new(
                        &std::env::var_os("HOME").unwrap_or_else(|| "/root".into()),
                    ))
                    .await,
                    Vec::new()
                ),
            },
            return
        ))
        .await
        .is_err()
    {
        return;
    }

    'outer: while let Some(Some(mut data)) = data_recv.recv().await {
        loop {
            tokio::select! {
                res = browser_handler_helper(&data) => {
                    let list = handle_error!(res, shared::BrowserList::default());
                    if socket_send.send(json_msg!(&list, continue)).await.is_err() {
                        break 'outer;
                    }
                    break;
                },
                recv = data_recv.recv() => match recv {
                    Some(Some(data_tmp)) => data = data_tmp,
                    _ => break 'outer,
                },
            }
        }
    }
}
