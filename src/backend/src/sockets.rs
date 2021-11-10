use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use nanoserde::{DeJson, SerJson};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{
    mpsc::{self, Receiver},
    Mutex,
};
use tokio::time::sleep;
use warp::ws::Message;

use crate::{systemdata, types};

async fn main_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    data_recv: &mut Receiver<Option<types::Request>>,
) {
    let handle = tokio::spawn(async move {
        let mut socket_send = socket_ptr.lock().await;
        loop {
            let _send = (*socket_send)
                .send(Message::text(SerJson::serialize_json(&types::SysData {
                    cpu: systemdata::cpu().await,
                    ram: systemdata::ram().await,
                    swap: systemdata::swap().await,
                    disk: systemdata::disk().await,
                    network: systemdata::network().await,
                })))
                .await;
        }
    });
    match data_recv.recv().await {
        Some(None) | None => {
            handle.abort();
        }
        Some(Some(_)) => {}
    }
}

async fn process_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    data_recv: &mut Receiver<Option<types::Request>>,
) {
    let handle = tokio::spawn(async move {
        loop {
            let mut socket_send = socket_ptr.lock().await;
            let _send = (*socket_send)
                .send(Message::text(SerJson::serialize_json(
                    &types::ProcessList {
                        processes: systemdata::processes().await,
                    },
                )))
                .await;
            sleep(Duration::from_secs(1)).await;
        }
    });
    loop {
        match data_recv.recv().await {
            Some(None) | None => {
                handle.abort();
                break;
            }
            Some(Some(data)) => {
                let process = heim::process::get(data.args[0].parse::<i32>().unwrap())
                    .await
                    .unwrap();
                log::info!(
                    "{}ing process {}",
                    data.cmd.trim_end_matches('e'),
                    process.pid()
                );
                match data.cmd.as_str() {
                    "terminate" => process.terminate().await.unwrap(),
                    "kill" => process.kill().await.unwrap(),
                    "suspend" => process.suspend().await.unwrap(),
                    "resume" => process.resume().await.unwrap(),
                    _ => (),
                }
            }
        }
    }
}

async fn software_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    data_recv: &mut Receiver<Option<types::Request>>,
) {
    let mut socket_send = socket_ptr.lock().await;
    let _send = (*socket_send)
        .send(Message::text(SerJson::serialize_json(
            &types::DPSoftwareList {
                software: systemdata::dpsoftware(),
                response: String::new(),
            },
        )))
        .await;
    loop {
        match data_recv.recv().await {
            Some(None) | None => {
                break;
            }
            Some(Some(data)) => {
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
                        .replace("", "");
                let _send = socket_send
                    .send(Message::text(SerJson::serialize_json(
                        &types::DPSoftwareList {
                            software: systemdata::dpsoftware(),
                            response: out,
                        },
                    )))
                    .await;
            }
        }
    }
}

async fn management_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    data_recv: &mut Receiver<Option<types::Request>>,
) {
    let mut socket_send = socket_ptr.lock().await;
    let _send = (*socket_send)
        .send(Message::text(SerJson::serialize_json(
            &systemdata::host().await,
        )))
        .await;
    loop {
        match data_recv.recv().await {
            Some(None) | None => {
                break;
            }
            Some(Some(data)) => {
                Command::new(data.cmd).spawn().unwrap();
            }
        }
    }
}

async fn service_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    data_recv: &mut Receiver<Option<types::Request>>,
) {
    let mut socket_send = socket_ptr.lock().await;
    let _send = (*socket_send)
        .send(Message::text(SerJson::serialize_json(
            &types::ServiceList {
                services: systemdata::services(),
            },
        )))
        .await;
    loop {
        match data_recv.recv().await {
            Some(None) | None => {
                break;
            }
            Some(Some(data)) => {
                Command::new("systemctl")
                    .args([data.cmd, (&*data.args[0]).to_string()])
                    .spawn()
                    .unwrap();
                let _send = (*socket_send)
                    .send(Message::text(SerJson::serialize_json(
                        &types::ServiceList {
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
            &types::BrowserList {
                contents: systemdata::browser_dir(std::path::Path::new(dir_path)),
            },
        )))
        .await;
}

async fn browser_handler(
    socket_ptr: Arc<Mutex<SplitSink<warp::ws::WebSocket, warp::ws::Message>>>,
    data_recv: &mut Receiver<Option<types::Request>>,
) {
    let mut socket_send = socket_ptr.lock().await;
    // Get initial listing of $HOME
    let _send = (*socket_send)
        .send(Message::text(SerJson::serialize_json(
            &types::BrowserList {
                contents: systemdata::browser_dir(std::path::Path::new(
                    &std::env::var_os("HOME").unwrap_or_else(|| "/root".into()),
                )),
            },
        )))
        .await;
    loop {
        match data_recv.recv().await {
            Some(None) | None => {
                break;
            }
            Some(Some(data)) => match data.cmd.as_str() {
                "cd" => {
                    let _send = (*socket_send)
                        .send(Message::text(SerJson::serialize_json(
                            &types::BrowserList {
                                contents: systemdata::browser_dir(std::path::Path::new(
                                    &data.args[0],
                                )),
                            },
                        )))
                        .await;
                }
                "open" => {
                    let _send = (*socket_send)
                        .send(Message::text(SerJson::serialize_json(
                            &types::BrowserFileData {
                                textdata: std::fs::read_to_string(std::path::Path::new(
                                    &data.args[0],
                                ))
                                .unwrap(),
                            },
                        )))
                        .await;
                }
                "img" => {
                    let _send = (*socket_send)
                        .send(Message::binary(std::fs::read(&data.args[0]).unwrap()))
                        .await;
                }
                "save" => {
                    std::fs::write(std::path::Path::new(&data.args[0]), &data.args[1]).unwrap();
                }
                "copy" => {
                    std::fs::copy(&data.args[0], format!("{} {}", &data.args[0], 2)).unwrap();
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
                _ => {}
            },
        }
    }
}

pub async fn socket_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();
    let (data_send, mut data_recv) = mpsc::channel(1);
    tokio::task::spawn(async move {
        let mut first_message = true;
        let mut req: types::Request;
        loop {
            let data = socket_recv.next().await.unwrap().unwrap();
            if data.is_close() {
                break;
            }
            req = DeJson::deserialize_json(data.to_str().unwrap()).unwrap();
            if req.cmd.is_empty() {
                if first_message {
                    first_message = false;
                } else {
                    // Quit out of handler
                    if data_send.send(None).await.is_err() {
                        break;
                    }
                }
            }
            // Send new page/data
            if data_send.send(Some(req.clone())).await.is_err() {
                break;
            }
        }
    });
    // Send global message (shown on all pages)
    let _send = socket_send
        .send(Message::text(
            SerJson::serialize_json(&systemdata::global()),
        ))
        .await;
    let socket_ptr = Arc::new(Mutex::new(socket_send));
    while let Some(Some(message)) = data_recv.recv().await {
        match message.page.as_str() {
            "/" => main_handler(Arc::clone(&socket_ptr), &mut data_recv).await,
            "/process" => {
                process_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/software" => {
                software_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/management" => {
                management_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/service" => {
                service_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/browser" => {
                browser_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            _ => {}
        }
    }
}
