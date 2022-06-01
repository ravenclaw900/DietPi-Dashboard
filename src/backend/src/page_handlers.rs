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
                let _send = (*socket_send)
                .send(Message::text(SerJson::serialize_json(&shared::SysData {
                    cpu: systemdata::cpu().await,
                    ram: systemdata::ram(),
                    swap: systemdata::swap(),
                    disk: systemdata::disk(),
                    network: systemdata::network(),
                })))
                .await;
            } => {}
        }
    }
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
                let _send = (*socket_send)
                .send(Message::text(SerJson::serialize_json(
                    &shared::ProcessList {
                        processes: systemdata::processes().await,
                    },
                )))
                .await;
                sleep(Duration::from_secs(1)).await;
            } => {}
            Some(data) = data_recv.recv() => {
                let pid = match data.args[0].parse::<u32>() {
                    Ok(pid) => pid,
                    Err(err) => {
                        log::warn!("Couldn't parse pid {}: {}", data.args[0], err);
                        continue;
                    }
                };
                let process = match psutil::process::Process::new(pid) {
                    Ok(process) => process,
                    Err(err) => {
                        log::warn!("Couldn't get process {}: {}", pid, err);
                        continue;
                    }
                };
                log::info!(
                    "{}ing process {}",
                    data.cmd.trim_end_matches('e'),
                    process.pid()
                );
                // Maybe make into an enum?
                if let Err(err) = match data.cmd.as_str() {
                    "terminate" => process.terminate(),
                    "kill" => process.kill(),
                    "suspend" => process.suspend(),
                    "resume" => process.resume(),
                    // Technically "no such argument", but needs to be a process error
                    _ => Err(psutil::process::ProcessError::NoSuchProcess{pid: process.pid()}),
                } {
                    log::warn!("Couldn't {} process {}: {}", data.cmd, process.pid(), err);
                }
            }
        }
    }
}

pub async fn software_handler(
    socket_ptr: SocketPtr,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    let software = systemdata::dpsoftware();
    let _send = (*socket_send)
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
            Some(data) = data_recv.recv() => {
                // We don't just want to run dietpi-software without args
                if data.args.is_empty() {
                    continue;
                }
                let mut cmd = Command::new("/boot/dietpi/dietpi-software");
                let mut arg_list = vec![data.cmd.as_str()];
                for element in &data.args {
                    arg_list.push(element.as_str());
                }
                log::info!("{}ing software with ID(s) {:?}", data.cmd, data.args);
                let out = match cmd.args(arg_list).output() {
                    Ok(out) => out.stdout,
                    Err(err) => {
                        log::warn!("Couldn't run DietPi-Software: {}", err);
                        continue
                    }
                };
                let out_str = match std::string::String::from_utf8(out) {
                    Ok(out_str) => out_str,
                    Err(err) => {
                        log::warn!("Invalid DietPi-Software output: {}", err);
                        continue;
                    }
                }
                // Replace bash color control character
                .replace('', "");
                let software = systemdata::dpsoftware();
                let _send = socket_send
                    .send(Message::text(SerJson::serialize_json(
                        &shared::DPSoftwareList {
                            uninstalled: software.0,
                            installed: software.1,
                            response: out_str,
                        },
                    )))
                    .await;
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
    let _send = (*socket_send)
        .send(Message::text(SerJson::serialize_json(&systemdata::host())))
        .await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => break,
            Some(data) = data_recv.recv() => {
                if let Err(err) = Command::new(&data.cmd).spawn() {
                    log::warn!("Couldn't spawn command {}: {}", data.cmd, err);
                }
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
    let _send = (*socket_send)
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
                    .args([data.cmd, (&*data.args[0]).to_string()])
                    .spawn() {
                        log::warn!("Couldn't spawn systemctl command: {}", err);
                        continue;
                    }
                let _send = (*socket_send)
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
    path: &str,
) {
    let dir_path = match path.rsplit_once('/') {
        Some(split_path) => split_path.0,
        None => {
            log::warn!("Couldn't split path {}", path);
            return;
        }
    };
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::BrowserList {
                contents: systemdata::browser_dir(std::path::Path::new(dir_path)),
            },
        )))
        .await;
}

pub async fn browser_handler(
    socket_ptr: SocketPtr,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    // Get initial listing of $HOME
    let _send = (*socket_send)
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
            Some(data) = data_recv.recv() => match data.cmd.as_str() {
                "cd" => {
                    let _send = (*socket_send)
                        .send(Message::text(SerJson::serialize_json(
                            &shared::BrowserList {
                                contents: systemdata::browser_dir(std::path::Path::new(
                                    &data.args[0],
                                )),
                            },
                        )))
                        .await;
                }
                "copy" => {
                    let mut num = 2;
                    while std::path::Path::new(&format!("{} {}", &data.args[0], num)).exists() {
                        num += 1;
                    }
                    let new_path = format!("{} {}", &data.args[0], num);
                    if let Err(err) = std::fs::copy(&data.args[0], &new_path) {
                        log::warn!("Couldn't copy file {} to {}: {}", &data.args[0], new_path, err);
                    }
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "rm" => {
                    if let Err(err) = std::fs::remove_file(&data.args[0]) {
                        log::warn!("Couldn't remove file {}: {}", &data.args[0], err);
                    }
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "rmdir" => {
                    if let Err(err) = std::fs::remove_dir_all(&data.args[0]) {
                        log::warn!("Couldn't remove directory {}: {}", &data.args[0], err);
                    }
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "mkdir" => {
                    if let Err(err) = std::fs::create_dir(&data.args[0]) {
                        log::warn!("Couldn't create directory {}: {}", &data.args[0], err);
                    }
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "mkfile" => {
                    if let Err(err) = std::fs::write(&data.args[0], "") {
                        log::warn!("Couldn't create file {}: {}", &data.args[0], err);
                    }
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "rename" => {
                    if let Err(err) = std::fs::copy(&data.args[0], &data.args[1]) {
                        log::warn!("Couldn't move file {} to {}: {}", &data.args[0], &data.args[1], err);
                    }
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "refresh" => {
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                _ => {}
            },
        }
    }
}
