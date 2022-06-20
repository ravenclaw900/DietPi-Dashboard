use anyhow::Context;
use futures::stream::SplitSink;
use futures::SinkExt;
use nanoserde::SerJson;
use std::process::Command;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;
use warp::ws::Message;

use crate::{handle_error, shared, systemdata};

type SocketSend = SplitSink<warp::ws::WebSocket, warp::ws::Message>;
type RecvChannel = Receiver<Option<shared::Request>>;

async fn main_handler_getter(
    cpu_collector: &mut psutil::cpu::CpuPercentCollector,
    net_collector: &mut psutil::network::NetIoCountersCollector,
    prev_data: &mut shared::NetData,
) -> anyhow::Result<shared::SysData> {
    Ok(shared::SysData {
        cpu: systemdata::cpu(cpu_collector).await?,
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
            Some(data) = data_recv.recv() => if data.is_none() {
                break;
            },
            _ = async {
                let _send = socket_send
                .send(Message::text(SerJson::serialize_json(&handle_error!(main_handler_getter(&mut cpu_collector, &mut net_collector, &mut prev_data).await, shared::SysData::default()))))
                .await;
            } => {}
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
    log::info!(
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
            Some(data) = data_recv.recv() => match data {
                Some(data) => handle_error!(process_handler_helper(&data)),
                None => break,
            },
            _ = async {
                let _send = socket_send
                .send(Message::text(SerJson::serialize_json(
                    &shared::ProcessList {
                        processes: handle_error!(systemdata::processes().await, Vec::new()),
                    },
                )))
                .await;
                sleep(Duration::from_secs(1)).await;
            } => {},
        }
    }
}

pub async fn software_handler_helper(
    data: &shared::Request,
    socket_send: &mut SocketSend,
) -> anyhow::Result<()> {
    // We don't just want to run dietpi-software without args
    anyhow::ensure!(!data.args.is_empty(), "Empty dietpi-software args");

    let mut cmd = Command::new("/boot/dietpi/dietpi-software");
    let mut arg_list = vec![data.cmd.as_str()];
    for element in &data.args {
        arg_list.push(element.as_str());
    }
    log::info!("{}ing software with ID(s) {:?}", data.cmd, data.args);
    let out = std::string::String::from_utf8(
        cmd.args(arg_list)
            .output()
            .context("Couldn't get DietPi-Software output")?
            .stdout,
    )
    .context("Invalid DietPi-Software output")?
    .replace('', "");

    let software = systemdata::dpsoftware()?;
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::DPSoftwareList {
                uninstalled: software.0,
                installed: software.1,
                response: out,
            },
        )))
        .await;

    Ok(())
}

pub async fn software_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    let software = handle_error!(systemdata::dpsoftware(), (Vec::new(), Vec::new()));
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::DPSoftwareList {
                uninstalled: software.0,
                installed: software.1,
                response: String::new(),
            },
        )))
        .await;
    while let Some(Some(data)) = data_recv.recv().await {
        handle_error!(software_handler_helper(&data, socket_send).await);
    }
}

pub async fn management_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(&handle_error!(
            systemdata::host(),
            shared::HostData::default()
        ))))
        .await;
    while let Some(Some(data)) = data_recv.recv().await {
        // Don't care about the Ok value, so remove it to make the type checker happy
        handle_error!(Command::new(&data.cmd)
            .spawn()
            .map(|_| ())
            .with_context(|| format!("Couldn't spawn command {}", &data.cmd)));
    }
}

pub async fn service_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::ServiceList {
                services: handle_error!(systemdata::services(), Vec::new()),
            },
        )))
        .await;
    while let Some(Some(data)) = data_recv.recv().await {
        handle_error!(Command::new("systemctl")
            .args([&data.cmd, data.args[0].as_str()])
            .spawn()
            .map(|_| ()) // Don't care about the Ok value, so remove it to make the type checker happy
            .with_context(|| format!("Couldn't {} service {}", &data.cmd, &data.args[0])));
        let _send = socket_send
            .send(Message::text(SerJson::serialize_json(
                &shared::ServiceList {
                    services: handle_error!(systemdata::services(), Vec::new()),
                },
            )))
            .await;
    }
}

async fn browser_refresh(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    path: &std::path::Path,
) -> anyhow::Result<()> {
    let dir_path = path
        .parent()
        .with_context(|| format!("Couldn't get parent of path {}", path.display()))?;
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::BrowserList {
                contents: systemdata::browser_dir(std::path::Path::new(dir_path))?,
            },
        )))
        .await;

    Ok(())
}

async fn browser_handler_helper(
    data: &shared::Request,
    socket_send: &mut SplitSink<warp::ws::WebSocket, Message>,
) -> anyhow::Result<()> {
    match data.cmd.as_str() {
        "cd" => {
            let _send = socket_send
                .send(Message::text(SerJson::serialize_json(
                    &shared::BrowserList {
                        contents: systemdata::browser_dir(std::path::Path::new(&data.args[0]))?,
                    },
                )))
                .await;
            return Ok(());
        }
        "copy" => {
            let mut num = 2;
            while std::path::Path::new(&format!("{} {}", &data.args[0], num)).exists() {
                num += 1;
            }
            std::fs::copy(&data.args[0], format!("{} {}", &data.args[0], num)).with_context(
                || format!("Couldn't copy file {0} to {0} {1}", &data.args[0], num),
            )?;
        }
        "rm" => {
            std::fs::remove_file(&data.args[0])
                .with_context(|| format!("Couldn't delete file at {}", &data.args[0]))?;
        }
        "rmdir" => {
            std::fs::remove_dir_all(&data.args[0])
                .with_context(|| format!("Couldn't delete directory at {}", &data.args[0]))?;
        }
        "mkdir" => {
            std::fs::create_dir(&data.args[0])
                .with_context(|| format!("Couldn't create directory at {}", &data.args[0]))?;
        }
        "mkfile" => {
            std::fs::write(&data.args[0], "")
                .with_context(|| format!("Couldn't create file at {}", &data.args[0]))?;
        }
        "rename" => {
            std::fs::rename(&data.args[0], &data.args[1]).with_context(|| {
                format!(
                    "Couldn't rename file {} to {}",
                    &data.args[0], &data.args[1]
                )
            })?;
        }
        _ => {}
    }

    browser_refresh(socket_send, std::path::Path::new(&data.args[0])).await?;

    Ok(())
}

pub async fn browser_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) {
    // Get initial listing of $HOME
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::BrowserList {
                contents: handle_error!(
                    systemdata::browser_dir(std::path::Path::new(
                        &std::env::var_os("HOME").unwrap_or_else(|| "/root".into()),
                    )),
                    Vec::new()
                ),
            },
        )))
        .await;

    while let Some(Some(data)) = data_recv.recv().await {
        handle_error!(browser_handler_helper(&data, socket_send).await);
    }
}
