use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use nanoserde::{DeJson, SerJson};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};
use std::{thread, time};
use tokio::sync::broadcast::{self, Receiver};
use warp::ws::Message;

use crate::{systemdata, types};

async fn main_handler(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit: &Arc<AtomicBool>,
) {
    loop {
        let _send = socket_send
            .send(Message::text(SerJson::serialize_json(&types::SysData {
                cpu: systemdata::cpu().await,
                ram: systemdata::ram().await,
                swap: systemdata::swap().await,
                disk: systemdata::disk().await,
                network: systemdata::network().await,
            })))
            .await;
        if quit.load(Relaxed) {
            quit.store(false, Relaxed);
            break;
        }
    }
}

async fn process_handler(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit: &Arc<AtomicBool>,
    data_recv: &mut Receiver<types::Request>,
) {
    loop {
        let _send = socket_send
            .send(Message::text(SerJson::serialize_json(
                &types::ProcessList {
                    processes: systemdata::processes().await,
                },
            )))
            .await;
        thread::sleep(time::Duration::from_secs(1));
        if quit.load(Relaxed) {
            quit.store(false, Relaxed);
            break;
        }
        match data_recv.try_recv() {
            Err(_) => {}
            Ok(data) => {
                let process = heim::process::get(data.args[0].parse::<i32>().unwrap())
                    .await
                    .unwrap();
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
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit: &Arc<AtomicBool>,
    data_recv: &mut Receiver<types::Request>,
) {
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &types::DPSoftwareList {
                software: systemdata::dpsoftware(),
                response: String::new(),
            },
        )))
        .await;
    loop {
        if quit.load(Relaxed) {
            quit.store(false, Relaxed);
            break;
        }
        match data_recv.try_recv() {
            Err(_) => {}
            Ok(data) => {
                // We don't just want to run dietpi-software without args
                if data.args.is_empty() {
                    continue;
                }
                let mut cmd = Command::new("/boot/dietpi/dietpi-software");
                let mut arg_list = vec![data.cmd.as_str()];
                for element in &data.args {
                    arg_list.push(element.as_str());
                }
                let out =
                    std::string::String::from_utf8(cmd.args(arg_list).output().unwrap().stdout)
                        .unwrap();
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
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit: &Arc<AtomicBool>,
    data_recv: &mut Receiver<types::Request>,
) {
    loop {
        let _send = socket_send
            .send(Message::text(SerJson::serialize_json(
                &systemdata::host().await,
            )))
            .await;
        thread::sleep(time::Duration::from_secs(1));
        if quit.load(Relaxed) {
            quit.store(false, Relaxed);
            break;
        }
        match data_recv.try_recv() {
            Err(_) => {}
            Ok(data) => {
                Command::new(data.cmd).spawn().unwrap();
            }
        }
    }
}

async fn service_handler(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit: &Arc<AtomicBool>,
    data_recv: &mut Receiver<types::Request>,
) {
    loop {
        let _send = socket_send
            .send(Message::text(SerJson::serialize_json(
                &types::ServiceList {
                    services: systemdata::services(),
                },
            )))
            .await;
        if quit.load(Relaxed) {
            quit.store(false, Relaxed);
            break;
        }
        match data_recv.try_recv() {
            Err(_) => {}
            Ok(data) => {
                Command::new("systemctl")
                    .args([data.cmd, (&*data.args[0]).to_string()])
                    .spawn()
                    .unwrap();
            }
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}

pub async fn socket_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();
    let (data_send, mut data_recv) = broadcast::channel(1);
    let quit = Arc::new(AtomicBool::new(false));
    let quit_clone = Arc::clone(&quit);
    tokio::task::spawn(async move {
        let mut first_message = true;
        let mut req: types::Request;
        loop {
            let data = socket_recv.next().await.unwrap().unwrap();
            if data.is_close() {
                break;
            }
            req = DeJson::deserialize_json(data.to_str().unwrap()).unwrap();
            data_send.send(req.clone()).unwrap();
            if req.cmd.is_empty() {
                if first_message {
                    first_message = false;
                } else {
                    quit.swap(true, Relaxed);
                }
            }
        }
    });
    // Send global message (shown on all pages)
    let _send = socket_send
        .send(Message::text(
            SerJson::serialize_json(&systemdata::global()),
        ))
        .await;
    while let Ok(message) = data_recv.recv().await {
        match message.page.as_str() {
            "/" => main_handler(&mut socket_send, &quit_clone).await,
            "/process" => {
                process_handler(&mut socket_send, &quit_clone, &mut data_recv).await;
            }
            "/software" => {
                software_handler(&mut socket_send, &quit_clone, &mut data_recv).await;
            }
            "/management" => {
                management_handler(&mut socket_send, &quit_clone, &mut data_recv).await;
            }
            "/service" => {
                service_handler(&mut socket_send, &quit_clone, &mut data_recv).await;
            }
            _ => {}
        }
    }
}
