use anyhow::Context;
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;
use tracing::instrument;

use crate::{
    handle_error,
    shared::{self, RequestTypes, SocketSend},
    systemdata,
};

type RecvChannel = Receiver<Option<shared::RequestTypes>>;

#[instrument(level = "debug", skip_all)]
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

// Return true if error was related to websocket, false otherwise
#[instrument(skip_all)]
pub async fn main_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) -> bool {
    let mut cpu_collector = handle_error!(
        psutil::cpu::CpuPercentCollector::new().context("Couldn't init cpu collector"),
        return false
    );

    let mut net_collector = psutil::network::NetIoCountersCollector::default();
    let mut prev_data = net_collector.net_io_counters().map_or_else(
        |_err| {
            tracing::debug!("Couldn't get original network counter data, starting with u64::MAX");
            shared::NetData {
                received: u64::MAX,
                sent: u64::MAX,
            }
        },
        |counters| shared::NetData {
            received: counters.bytes_recv(),
            sent: counters.bytes_sent(),
        },
    );

    loop {
        tokio::select! {
            biased;
            data = data_recv.recv() => if let Some(Some(_)) = data {} else { return false },
            res = socket_send
            .send(shared::BackendData::Statistic(handle_error!(main_handler_getter(&mut cpu_collector, &mut net_collector, &mut prev_data), shared::SysData::default())))
            => {
                sleep(Duration::from_secs(1)).await;
                if res.is_err() {
                    tracing::debug!("Socket send failed, returning");
                    return true;
                }
            },
        }
    }
}

#[instrument(level = "debug", skip_all)]
fn process_handler_helper(cmd: &str, arg: Option<&str>) -> anyhow::Result<()> {
    if let Some(arg) = arg {
        let process = psutil::process::Process::new(
            arg.parse::<u32>()
                .with_context(|| format!("Invalid pid {arg}"))?,
        )
        .with_context(|| format!("Couldn't make process from pid {arg}"))?;
        tracing::info!("{}ing process {}", cmd.trim_end_matches('e'), process.pid());
        match cmd {
            "terminate" => process.terminate(),
            "kill" => process.kill(),
            "suspend" => process.suspend(),
            "resume" => process.resume(),
            _ => Ok(()),
        }
        .with_context(|| format!("Couldn't {cmd} process {}", process.pid()))?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("No argument"))
    }
}

#[instrument(skip_all)]
pub async fn process_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) -> bool {
    loop {
        tokio::select! {
            biased;
            data = data_recv.recv() => match data {
                Some(Some(RequestTypes::Cmd { cmd, args: Some(args) })) => handle_error!(process_handler_helper(&cmd, args.first().map(String::as_str))),
                Some(Some(_)) => {}
                _ => return false,
            },
            res = socket_send
                .send(shared::BackendData::Process(shared::ProcessList {
                        processes: handle_error!(systemdata::processes().await, Vec::new()),
                    }
                )) => {
                    sleep(Duration::from_secs(1)).await;
                    if res.is_err() {
                        tracing::debug!("Socket send failed, returning");
                        return true;
                    }
                },
        }
    }
}

#[instrument(level = "debug", skip_all)]
pub async fn software_handler_helper(
    cmd: &str,
    args: &[String],
) -> anyhow::Result<shared::DPSoftwareList> {
    // We don't just want to run dietpi-software without args
    anyhow::ensure!(!args.is_empty(), "Empty DietPi-Software args");

    let mut software_cmd = tokio::process::Command::new("/boot/dietpi/dietpi-software");
    let mut arg_list = vec![cmd];
    for element in args {
        arg_list.push(element.as_str());
    }
    tracing::info!("{}ing software with ID(s) {:?}", cmd, args);
    let out = shared::remove_color_codes(
        std::str::from_utf8(
            &software_cmd
                .args(arg_list)
                .output()
                .await
                .context("Couldn't get DietPi-Software output")?
                .stdout,
        )
        .context("Invalid DietPi-Software output")?,
    );

    let software = systemdata::dpsoftware().await?;
    Ok(shared::DPSoftwareList {
        uninstalled: software.0,
        installed: software.1,
        response: Some(out),
    })
}

#[instrument(skip_all)]
pub async fn software_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) -> bool {
    let software = handle_error!(systemdata::dpsoftware().await, (Vec::new(), Vec::new()));
    if socket_send
        .send(shared::BackendData::Software(shared::DPSoftwareList {
            uninstalled: software.0,
            installed: software.1,
            response: None,
        }))
        .await
        .is_err()
    {
        tracing::debug!("Socket send failed, returning");
        return true;
    }
    while let Some(Some(data)) = data_recv.recv().await {
        if let RequestTypes::Cmd {
            cmd,
            args: Some(args),
        } = data
        {
            if socket_send
                .send(shared::BackendData::Software(handle_error!(
                    software_handler_helper(&cmd, &args).await,
                    shared::DPSoftwareList::default()
                )))
                .await
                .is_err()
            {
                tracing::debug!("Socket send failed, returning");
                return true;
            }
        }
    }
    false
}

#[instrument(skip_all)]
pub async fn management_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) -> bool {
    if socket_send
        .send(shared::BackendData::Management(handle_error!(
            systemdata::host().await,
            shared::HostData::default()
        )))
        .await
        .is_err()
    {
        tracing::debug!("Socket send failed, returning");
        return true;
    }
    while let Some(Some(data)) = data_recv.recv().await {
        if let RequestTypes::Cmd { cmd, args: _ } = data {
            tracing::info!("Running command {}", &cmd);
            // Don't care about the Ok value, so remove it to make the type checker happy
            handle_error!(Command::new(&cmd)
                .spawn()
                .map(|_| ())
                .with_context(|| format!("Couldn't spawn command {}", &cmd)));
        }
    }
    false
}

#[instrument(skip_all)]
pub async fn service_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) -> bool {
    if socket_send
        .send(shared::BackendData::Service(shared::ServiceList {
            services: handle_error!(systemdata::services().await, Vec::new()),
        }))
        .await
        .is_err()
    {
        tracing::debug!("Socket send failed, returning");
        return true;
    }
    while let Some(Some(data)) = data_recv.recv().await {
        if let RequestTypes::Cmd {
            cmd,
            args: Some(args),
        } = data
        {
            if let Some(arg) = args.first() {
                handle_error!(Command::new("systemctl")
                    .args([&cmd, arg])
                    .spawn()
                    .map(|_| ()) // Don't care about the Ok value, so remove it to make the type checker happy
                    .with_context(|| format!("Couldn't {} service {arg}", &cmd)));
                if socket_send
                    .send(shared::BackendData::Service(shared::ServiceList {
                        services: handle_error!(systemdata::services().await, Vec::new()),
                    }))
                    .await
                    .is_err()
                {
                    tracing::debug!("Socket send failed, returning");
                    return true;
                }
            }
        }
    }
    false
}

async fn browser_refresh(path: &std::path::Path) -> anyhow::Result<shared::BrowserList> {
    let dir_path = path
        .parent()
        .with_context(|| format!("Couldn't get parent of path {}", path.display()))?;

    Ok(shared::BrowserList {
        contents: systemdata::browser_dir(std::path::Path::new(dir_path)).await?,
    })
}

#[instrument(level = "debug", skip_all)]
async fn browser_handler_helper(cmd: &str, args: &[String]) -> anyhow::Result<shared::BrowserList> {
    use tokio::fs;

    tracing::debug!("Command is {}", cmd);

    if let Some(arg) = args.first() {
        match cmd {
            "cd" => {
                return Ok(shared::BrowserList {
                    contents: systemdata::browser_dir(std::path::Path::new(arg)).await?,
                });
            }
            "copy" => {
                let mut num = 2;
                while std::path::Path::new(&format!("{arg} {num}")).exists() {
                    num += 1;
                }
                fs::copy(arg, format!("{arg} {num}"))
                    .await
                    .with_context(|| format!("Couldn't copy file {arg} to {arg} {num}"))?;
            }
            "rm" => {
                fs::remove_file(arg)
                    .await
                    .with_context(|| format!("Couldn't delete file at {arg}"))?;
            }
            "rmdir" => {
                fs::remove_dir_all(arg)
                    .await
                    .with_context(|| format!("Couldn't delete directory at {arg}"))?;
            }
            "mkdir" => {
                fs::create_dir(arg)
                    .await
                    .with_context(|| format!("Couldn't create directory at {arg}"))?;
            }
            "mkfile" => {
                fs::write(arg, "")
                    .await
                    .with_context(|| format!("Couldn't create file at {arg}"))?;
            }
            "rename" => {
                if let Some(arg1) = args.get(1) {
                    fs::rename(arg, arg1)
                        .await
                        .with_context(|| format!("Couldn't rename file {arg} to {arg1}"))?;
                } else {
                    return Err(anyhow::anyhow!("No second argument"));
                }
            }
            _ => tracing::debug!("Got command {}, not handling", cmd),
        }
        browser_refresh(std::path::Path::new(arg)).await
    } else {
        Err(anyhow::anyhow!("No argument"))
    }
}

#[instrument(skip_all)]
pub async fn browser_handler(socket_send: &mut SocketSend, data_recv: &mut RecvChannel) -> bool {
    // Get initial listing of $HOME
    if socket_send
        .send(shared::BackendData::Browser(shared::BrowserList {
            contents: handle_error!(
                systemdata::browser_dir(std::path::Path::new(
                    &std::env::var_os("HOME").unwrap_or_else(|| "/root".into()),
                ))
                .await,
                Vec::new()
            ),
        }))
        .await
        .is_err()
    {
        tracing::debug!("Socket send failed, returning");
        return true;
    }

    'outer: while let Some(Some(mut data)) = data_recv.recv().await {
        loop {
            if let RequestTypes::Cmd {
                cmd,
                args: Some(args),
            } = &data
            {
                tokio::select! {
                    res = browser_handler_helper(cmd, args) => {
                        if socket_send.send(shared::BackendData::Browser(handle_error!(res, shared::BrowserList::default()))).await.is_err() {
                            tracing::debug!("Socket send failed, returning");
                            return true;
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
    false
}
