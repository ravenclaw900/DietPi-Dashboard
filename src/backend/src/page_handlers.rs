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

pub async fn main_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => {break}
            _ = async {
                let _send = (*socket_send)
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

pub async fn process_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    data_recv: &mut Receiver<shared::Request>,
    quit: &Arc<Notify>,
) {
    let mut socket_send = socket_ptr.lock().await;
    loop {
        tokio::select! {
            biased;
            _ = quit.notified() => {break}
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
                let process =
                    psutil::process::Process::new(data.args[0].parse::<u32>().unwrap()).unwrap();
                log::info!(
                    "{}ing process {}",
                    data.cmd.trim_end_matches('e'),
                    process.pid()
                );
                match data.cmd.as_str() {
                    "terminate" => process.terminate().unwrap(),
                    "kill" => process.kill().unwrap(),
                    "suspend" => process.suspend().unwrap(),
                    "resume" => process.resume().unwrap(),
                    _ => (),
                }
            }
        }
    }
}

pub async fn software_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
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
            _ = quit.notified() => {break}
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
                let out =
                    std::string::String::from_utf8(cmd.args(arg_list).output().unwrap().stdout)
                        .unwrap()
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
            }
        }
    }
}

pub async fn management_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
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
            _ = quit.notified() => {break}
            Some(data) = data_recv.recv() => {
                Command::new(data.cmd).spawn().unwrap();
            }
        }
    }
}

pub async fn service_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
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
            _ = quit.notified() => {break}
            Some(data) = data_recv.recv() =>  {
                Command::new("systemctl")
                    .args([data.cmd, (&*data.args[0]).to_string()])
                    .spawn()
                    .unwrap();
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
    let dir_path = path.rsplit_once('/').unwrap().0;
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &shared::BrowserList {
                contents: systemdata::browser_dir(std::path::Path::new(dir_path)),
            },
        )))
        .await;
}

pub async fn browser_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
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
            _ = quit.notified() => {break}
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
                    std::fs::copy(&data.args[0], format!("{} {}", &data.args[0], num)).unwrap();
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "rm" => {
                    std::fs::remove_file(&data.args[0]).unwrap();
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "rmdir" => {
                    std::fs::remove_dir_all(&data.args[0]).unwrap();
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "mkdir" => {
                    std::fs::create_dir(&data.args[0]).unwrap();
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "mkfile" => {
                    std::fs::write(&data.args[0], "").unwrap();
                    browser_refresh(&mut *socket_send, &data.args[0]).await;
                }
                "rename" => {
                    std::fs::rename(&data.args[0], &data.args[1]).unwrap();
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
