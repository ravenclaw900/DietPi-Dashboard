use futures::{SinkExt, StreamExt};
use pty_process::Command;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::RwLock;
use warp::ws::Message;

#[derive(serde::Deserialize)]
struct TTYSize {
    cols: u16,
    rows: u16,
}

pub async fn term_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();

    let cmd = Arc::new(RwLock::new(
        std::process::Command::new("/bin/bash")
            .spawn_pty(None)
            .unwrap(),
    ));
    let cmd_read = Arc::clone(&cmd);
    let cmd_write = Arc::clone(&cmd);

    let stop_thread_write = Arc::new(AtomicBool::new(false));
    let stop_thread_read = Arc::clone(&stop_thread_write);

    let pty_writer = tokio::spawn(async move {
        while let Some(Ok(data)) = socket_recv.next().await {
            let lock = cmd_write.read().await;
            if stop_thread_write.load(Ordering::Relaxed) {
                break;
            }
            if data.is_text() && data.to_str().unwrap().get(..4) == Some("size") {
                let json: TTYSize = serde_json::from_str(&data.to_str().unwrap()[4..]).unwrap();
                lock.deref()
                    .resize_pty(&pty_process::Size::new(json.rows, json.cols))
                    .unwrap();
                continue;
            }
            lock.deref().pty().write_all(data.as_bytes()).unwrap();
        }
        stop_thread_write.swap(true, Ordering::Relaxed);
        // Stop reader
        cmd_write
            .read()
            .await
            .deref()
            .pty()
            .write_all("exit".as_bytes());
    });

    let pty_reader = tokio::spawn(async move {
        loop {
            let mut data = [0; 1024];
            let lock = cmd_read.read().await;
            match lock.deref().pty().read(&mut data) {
                Ok(_) => {}
                Err(_) => break,
            };
            if stop_thread_read.load(Ordering::Relaxed) {
                break;
            }
            socket_send.send(Message::binary(data)).await.unwrap();
        }
        stop_thread_read.swap(true, Ordering::Relaxed);
        // Writer won't exit until page is changed/closed
    });

    // Wait for threads to exit
    tokio::join!(pty_writer, pty_reader);

    // Process should be safe to kill after exiting
    cmd.write().await.deref_mut().kill().unwrap();
}
