use futures::{SinkExt, StreamExt};
use pty_process::Command;
use std::io::{Read, Write};
use std::sync::Arc;
use warp::ws::Message;

#[derive(serde::Deserialize)]
struct TTYSize {
    cols: u16,
    rows: u16,
}

pub async fn terminal_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();

    let cmd = Arc::new(
        std::process::Command::new("/bin/bash")
            .spawn_pty(None)
            .unwrap(),
    );

    let cmd_write = cmd.clone();

    let cmd_read = cmd.clone();

    tokio::spawn(async move {
        loop {
            let data = socket_recv.next().await.unwrap().unwrap();
            if data.to_str().unwrap().get(..4) == Some("size") {
                let json: TTYSize = serde_json::from_str(&data.to_str().unwrap()[4..]).unwrap();
                cmd_write
                    .resize_pty(&pty_process::Size::new(json.rows, json.cols))
                    .unwrap();
                continue;
            }
            cmd_write.pty().write_all(data.as_bytes()).unwrap();
        }
    });

    tokio::spawn(async move {
        loop {
            let mut data = [0; 2048];
            let num_read = cmd_read.pty().read(&mut data).unwrap();
            if num_read == 0 {
                continue;
            }
            socket_send.send(Message::binary(data)).await.unwrap();
        }
    });
}
