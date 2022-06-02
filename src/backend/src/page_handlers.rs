use anyhow::Context;
use futures::stream::SplitSink;
use futures::SinkExt;
use nanoserde::SerJson;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::sync::{mpsc::Receiver, Mutex};
use tokio::time::sleep;
use warp::ws::Message;

use crate::{shared, systemdata};

type SocketPtr = Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>;

pub async fn main_handler(socket_ptr: SocketPtr, quit: &Arc<Notify>) {
    let mut socket_send = socket_ptr.lock().await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => break,
            _ = async {
                let _send = socket_send
                .send(Message::text(SerJson::serialize_json(&shared::SysData {
                    cpu: systemdata::cpu().await,
                    ram: systemdata::ram(),
                    swap: systemdata::swap(),
                    disk: systemdata::disk(),
                    network: systemdata::network(),
                    temp: systemdata::temp(),
                })))
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
        "terminate" => process.terminate()?,
        "kill" => process.kill()?,
        "suspend" => process.suspend()?,
        "resume" => process.resume()?,
        _ => (),
    };
    Ok(())
}

pub async fn process_handler(
    socket_ptr: SocketPtr,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => break,
            _ = async {
                let _send = socket_send
                .send(Message::text(SerJson::serialize_json(
                    &shared::ProcessList {
                        processes: systemdata::processes().await,
                    },
                )))
                .await;
                sleep(Duration::from_secs(1)).await;
            } => {}
            Some(data) = data_recv.recv() => if let Err(err) = process_handler_helper(&data) {
                log::warn!("{}: {}", err, err.root_cause());
            }
        }
    }
}

pub async fn software_handler_helper(
    data: &shared::Request,
    socket_send: &mut tokio::sync::MutexGuard<'_, SplitSink<warp::ws::WebSocket, Message>>,
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

    let software = systemdata::dpsoftware();
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

pub async fn software_handler(
    socket_ptr: SocketPtr,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    let software = systemdata::dpsoftware();
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::DPSoftwareList {
                uninstalled: software.0,
                installed: software.1,
                response: String::new(),
            },
        )))
        .await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => break,
            Some(data) = data_recv.recv() => if let Err(err) = software_handler_helper(&data, &mut socket_send).await {
                    log::warn!("{}: {}", err, err.root_cause());
            }
        }
    }
}

pub async fn management_handler(
    socket_ptr: SocketPtr,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(&systemdata::host())))
        .await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => break,
            Some(data) = data_recv.recv() => if let Err(err) = Command::new(&data.cmd).spawn() {
                    log::warn!("Couldn't spawn command {}: {}", &data.cmd, err);
            }
        }
    }
}

pub async fn service_handler(
    socket_ptr: SocketPtr,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::ServiceList {
                services: systemdata::services(),
            },
        )))
        .await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => break,
            Some(data) = data_recv.recv() =>  {
                if let Err(err) = Command::new("systemctl")
                    .args([&data.cmd, data.args[0].as_str()])
                    .spawn() {
                        log::warn!("Couldn't {} service {}: {}", &data.cmd, &data.args[0], err);
                    }
                let _send = socket_send
                    .send(Message::text(SerJson::serialize_json(
                        &shared::ServiceList {
                            services: systemdata::services(),
                        },
                    )))
                    .await;
            }
        }
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
                contents: systemdata::browser_dir(std::path::Path::new(dir_path)),
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

    // 'refresh' and 'cd' covered here
    // TODO: remove 'refresh', only use 'cd'
    browser_refresh(socket_send, std::path::Path::new(&data.args[0])).await?;

    Ok(())
}

pub async fn browser_handler(
    socket_ptr: SocketPtr,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    // Get initial listing of $HOME
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::BrowserList {
                contents: systemdata::browser_dir(std::path::Path::new(
                    &std::env::var_os("HOME").unwrap_or_else(|| "/root".into()),
                )),
            },
        )))
        .await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => break,
            Some(data) = data_recv.recv() => if let Err(err) = browser_handler_helper(&data, &mut *socket_send).await {
                log::warn!("{}: {}", err, err.root_cause());
            }
        }
    }
}
