use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use std::process::Command;
use std::{thread, time};
use tokio::sync::broadcast::{self, Receiver};
use warp::ws::Message;

use crate::{systemdata, types};

async fn main_handler(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit_recv: &mut Receiver<bool>,
) {
    loop {
        let _send = socket_send
            .send(Message::text(
                serde_json::to_string(&types::SysData {
                    cpu: systemdata::cpu(),
                    ram: systemdata::ram(),
                    swap: systemdata::swap(),
                    disk: systemdata::disk(),
                    network: systemdata::network(),
                })
                .unwrap(),
            ))
            .await;
        thread::sleep(time::Duration::from_millis(500));
        match quit_recv.try_recv() {
            Err(_) => {}
            Ok(_) => break,
        }
    }
}

async fn process_handler(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit_recv: &mut Receiver<bool>,
    data_recv: &mut Receiver<types::Request>,
) {
    loop {
        let _send = socket_send
            .send(Message::text(
                serde_json::to_string(&types::ProcessList {
                    processes: systemdata::processes(),
                })
                .unwrap(),
            ))
            .await;
        thread::sleep(time::Duration::from_millis(1000));
        match quit_recv.try_recv() {
            Err(_) => {}
            Ok(_) => break,
        }
        match data_recv.try_recv() {
            Err(_) => {}
            Ok(data) => {
                let process =
                    psutil::process::Process::new(data.args[0].parse::<u32>().unwrap()).unwrap();
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

async fn software_handler(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit_recv: &mut Receiver<bool>,
    data_recv: &mut Receiver<types::Request>,
) {
    let _send = socket_send
        .send(Message::text(
            serde_json::to_string(&types::DPSoftwareList {
                software: systemdata::dpsoftware(),
                response: String::new(),
            })
            .unwrap(),
        ))
        .await;
    loop {
        match data_recv.try_recv() {
            Err(_) => {}
            Ok(data) => {
                let mut cmd = Command::new("/boot/dietpi/dietpi-software");
                let mut arg_list = vec![data.cmd.as_str()];
                for element in &data.args {
                    arg_list.push(element.as_str());
                }
                let out =
                    std::string::String::from_utf8(cmd.args(arg_list).output().unwrap().stdout)
                        .unwrap();
                let _send = socket_send
                    .send(Message::text(
                        serde_json::to_string(&types::DPSoftwareList {
                            software: systemdata::dpsoftware(),
                            response: out,
                        })
                        .unwrap(),
                    ))
                    .await;
            }
        }
        match quit_recv.try_recv() {
            Err(_) => {}
            Ok(_) => break,
        }
    }
}

async fn management_handler(
    socket_send: &mut SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    quit_recv: &mut Receiver<bool>,
    data_recv: &mut Receiver<types::Request>,
) {
    let _send = socket_send
        .send(Message::text(
            serde_json::to_string(&systemdata::host()).unwrap(),
        ))
        .await;
    loop {
        match data_recv.try_recv() {
            Err(_) => {}
            Ok(data) => {
                Command::new(data.cmd).spawn().unwrap();
            }
        }
        match quit_recv.try_recv() {
            Err(_) => {}
            Ok(_) => break,
        }
    }
}

pub async fn socket_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();
    let (data_send, mut data_recv) = broadcast::channel(1);
    let (quit_send, mut quit_recv) = broadcast::channel(1);
    tokio::task::spawn(async move {
        let mut first_message = true;
        let mut req: types::Request;
        loop {
            let data = socket_recv.next().await.unwrap().unwrap();
            if data.is_close() {
                break;
            }
            req = serde_json::from_str(data.to_str().unwrap()).unwrap();
            data_send.send(req.clone()).unwrap();
            if req.cmd.is_empty() {
                if first_message {
                    first_message = false;
                } else {
                    quit_send.send(true).unwrap();
                }
            }
        }
    });
    loop {
        let message = data_recv.recv().await.unwrap();
        match message.page.as_str() {
            "/" => main_handler(&mut socket_send, &mut quit_recv).await,
            "/process" => process_handler(&mut socket_send, &mut quit_recv, &mut data_recv).await,
            "/software" => software_handler(&mut socket_send, &mut quit_recv, &mut data_recv).await,
            "/management" => {
                management_handler(&mut socket_send, &mut quit_recv, &mut data_recv).await;
            }
            _ => {}
        }
    }
}
