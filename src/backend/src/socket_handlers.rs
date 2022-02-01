use futures::{SinkExt, StreamExt};
use nanoserde::{DeJson, SerJson};
use pty_process::Command;
use std::io::{Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};
use tokio::sync::RwLock;
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

use crate::{page_handlers, shared, systemdata, CONFIG};

fn validate_token(token: &str) -> bool {
    let key = jwts::jws::Key::new(&crate::CONFIG.secret, jwts::jws::Algorithm::HS256);
    let verified: jwts::jws::Token<jwts::Claims>;
    if let Ok(token) = jwts::jws::Token::verify_with_key(token, &key) {
        verified = token;
    } else {
        log::error!("Couldn't verify token");
        return false;
    };
    let config = jwts::ValidationConfig {
        iat_validation: false,
        nbf_validation: false,
        exp_validation: true,
        expected_iss: Some("DietPi Dashboard".to_string()),
        expected_sub: None,
        expected_aud: None,
        expected_jti: None,
    };
    if verified.validate_claims(&config).is_err() {
        return false;
    }
    true
}

#[allow(clippy::too_many_lines)]
pub async fn socket_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();
    let (data_send, mut data_recv) = mpsc::channel(1);
    tokio::task::spawn(async move {
        let mut first_message = true;
        let mut req: shared::Request;
        while let Some(Ok(data)) = socket_recv.next().await {
            if data.is_close() {
                break;
            }
            let data_str;
            if let Ok(data_string) = data.to_str() {
                data_str = data_string;
            } else {
                log::error!("Couldn't convert received data to text");
                continue;
            }
            req = if let Ok(json) = DeJson::deserialize_json(data_str) {
                json
            } else {
                log::error!("Couldn't parse JSON");
                continue;
            };
            if CONFIG.pass && !validate_token(&req.token) && !first_message {
                if data_send.send(None).await.is_err() {
                    break;
                }
                data_send
                    .send(Some(shared::Request {
                        page: "/login".to_string(),
                        token: String::new(),
                        cmd: String::new(),
                        args: Vec::new(),
                    }))
                    .await
                    .unwrap();
                continue;
            }
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
            "/" => page_handlers::main_handler(Arc::clone(&socket_ptr), &mut data_recv).await,
            "/process" => {
                page_handlers::process_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/software" => {
                page_handlers::software_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/management" => {
                page_handlers::management_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/service" => {
                page_handlers::service_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/browser" => {
                page_handlers::browser_handler(Arc::clone(&socket_ptr), &mut data_recv).await;
            }
            "/login" => {
                // Internal poll, see other thread
                let _send = (*socket_ptr.lock().await)
                    .send(Message::text(SerJson::serialize_json(
                        &shared::TokenError { error: true },
                    )))
                    .await;
            }
            _ => {}
        }
    }
}

#[derive(DeJson)]
struct TTYSize {
    cols: u16,
    rows: u16,
}

pub async fn term_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();

    if crate::CONFIG.pass {
        let token = socket_recv.next().await.unwrap().unwrap();
        let token = token.to_str().unwrap();
        if token.get(..5) == Some("token") {
            if !validate_token(&token[5..]) {
                return;
            }
        } else {
            return;
        }
    }

    let cmd = Arc::new(RwLock::new(
        // Use hardcoded bash here until we have better support for other shells
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
            if stop_thread_write.load(Relaxed) {
                break;
            }
            if data.is_text() && data.to_str().unwrap().get(..4) == Some("size") {
                let json: TTYSize = DeJson::deserialize_json(&data.to_str().unwrap()[4..]).unwrap();
                (*lock)
                    .resize_pty(&pty_process::Size::new(json.rows, json.cols))
                    .unwrap();
                continue;
            }
            (*lock).pty().write_all(data.as_bytes()).unwrap();
        }
        stop_thread_write.store(true, Relaxed);
        // Stop reader
        (*cmd_write.read().await)
            .pty()
            .write_all("exit\n".as_bytes())
            .unwrap();
    });

    let pty_reader = tokio::spawn(async move {
        loop {
            let mut data = [0; 256];
            let lock = cmd_read.read().await;
            if (*lock).pty().read(&mut data).is_err() {
                break;
            };
            if stop_thread_read.load(Relaxed) {
                break;
            }
            socket_send
                .send(Message::binary(data.split(|num| num == &0).next().unwrap()))
                .await
                .unwrap();
        }
        stop_thread_read.store(true, Relaxed);
        // Writer won't exit until page is changed/closed
    });

    // Wait for threads to exit
    tokio::try_join!(pty_writer, pty_reader).unwrap();

    // Reap PID
    (*cmd.write().await).wait().unwrap();

    log::info!("Closed terminal");
}

pub async fn file_handler(mut socket: warp::ws::WebSocket) {
    let mut req: shared::FileRequest;
    while let Some(Ok(data)) = socket.next().await {
        if data.is_close() {
            break;
        }
        let data_str;
        if let Ok(data_string) = data.to_str() {
            data_str = data_string;
        } else {
            log::error!("Couldn't convert received data to text");
            continue;
        }
        req = if let Ok(json) = DeJson::deserialize_json(data_str) {
            json
        } else {
            log::error!("Couldn't parse JSON");
            continue;
        };
        if CONFIG.pass && !validate_token(&req.token) {
            continue;
        }

        match req.cmd.as_str() {
            "open" => {
                let _send = socket
                    .send(Message::text(
                        std::fs::read_to_string(std::path::Path::new(&req.path)).unwrap(),
                    ))
                    .await;
            }
            // Technically works for both files and directories
            "dl" => {
                let mut buf = Vec::new();
                let src_path = std::path::Path::new(&req.path);
                {
                    let mut zip_file = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
                    let mut file_buf = Vec::new();
                    for entry in walkdir::WalkDir::new(&req.path) {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        let name = std::path::Path::new(src_path.file_name().unwrap())
                            .join(path.strip_prefix(src_path).unwrap());
                        if path.is_file() {
                            zip_file
                                .start_file(
                                    name.to_str().unwrap(),
                                    zip::write::FileOptions::default(),
                                )
                                .unwrap();
                            let mut f = std::fs::File::open(path).unwrap();
                            f.read_to_end(&mut file_buf).unwrap();
                            zip_file.write_all(&file_buf).unwrap();
                            file_buf.clear();
                        } else if !name.as_os_str().is_empty() {
                            zip_file
                                .add_directory(
                                    name.to_str().unwrap(),
                                    zip::write::FileOptions::default(),
                                )
                                .unwrap();
                        }
                    }
                    zip_file.finish().unwrap();
                }
                let _send = socket
                    .send(Message::binary(
                        //std::fs::read(std::path::Path::new(&req.path)).unwrap(),
                        buf,
                    ))
                    .await;
            }
            "save" => std::fs::write(std::path::Path::new(&req.path), &req.arg).unwrap(),
            _ => {}
        }
    }
}
