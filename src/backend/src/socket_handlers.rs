use futures::{SinkExt, StreamExt};
use nanoserde::{DeJson, SerJson};
use pty_process::Command;
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

use crate::{page_handlers, shared, systemdata, CONFIG};

fn validate_token(token: &str) -> bool {
    let mut validator = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validator.set_issuer(&["DietPi Dashboard"]);
    validator.set_required_spec_claims(&["exp", "nbf"]);
    if jsonwebtoken::decode::<shared::JWTClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(CONFIG.secret.as_ref()),
        &validator,
    )
    .is_err()
    {
        return false;
    }
    true
}

#[allow(clippy::too_many_lines)]
pub async fn socket_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();
    let (data_send, mut data_recv) = mpsc::channel(1);
    let quit = Arc::new(tokio::sync::Notify::new());
    let quit_send = Arc::clone(&quit);
    tokio::task::spawn(async move {
        let mut req: shared::Request;
        while let Some(Ok(data)) = socket_recv.next().await {
            if data.is_close() {
                break;
            }
            let data_str = if let Ok(data_string) = data.to_str() {
                data_string
            } else {
                log::error!("Couldn't convert received data to text");
                continue;
            };
            req = if let Ok(json) = DeJson::deserialize_json(data_str) {
                json
            } else {
                log::error!("Couldn't parse JSON");
                continue;
            };
            if CONFIG.pass && !validate_token(&req.token) {
                quit_send.notify_waiters();
                data_send
                    .send(shared::Request {
                        page: "/login".to_string(),
                        token: String::new(),
                        cmd: String::new(),
                        args: Vec::new(),
                    })
                    .await
                    .unwrap();
                continue;
            }
            if req.cmd.is_empty() {
                // Quit out of handler
                quit_send.notify_waiters();
            }
            // Send new page/data
            if data_send.send(req.clone()).await.is_err() {
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
    while let Some(message) = data_recv.recv().await {
        match message.page.as_str() {
            "/" => page_handlers::main_handler(Arc::clone(&socket_ptr), &quit).await,
            "/process" => {
                page_handlers::process_handler(Arc::clone(&socket_ptr), &mut data_recv, &quit)
                    .await;
            }
            "/software" => {
                page_handlers::software_handler(Arc::clone(&socket_ptr), &mut data_recv, &quit)
                    .await;
            }
            "/management" => {
                page_handlers::management_handler(Arc::clone(&socket_ptr), &mut data_recv, &quit)
                    .await;
            }
            "/service" => {
                page_handlers::service_handler(Arc::clone(&socket_ptr), &mut data_recv, &quit)
                    .await;
            }
            "/browser" => {
                page_handlers::browser_handler(Arc::clone(&socket_ptr), &mut data_recv, &quit)
                    .await;
            }
            "/login" => {
                // Internal poll, see other thread
                let _send = (*socket_ptr.lock().await)
                    .send(Message::text(SerJson::serialize_json(
                        &shared::TokenError { error: true },
                    )))
                    .await;
            }
            _ => {
                log::debug!("Got page {}, not handling", message.page);
            }
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
    let cmd_write = Arc::clone(&cmd);
    let cmd_clone = Arc::clone(&cmd);

    let quit_send = Arc::new(tokio::sync::Notify::new());
    let quit_recv = Arc::clone(&quit_send);

    let (send, mut recv) = mpsc::channel(2);

    tokio::spawn(async move {
        loop {
            let cmd_read = Arc::clone(&cmd_clone);
            let lock = cmd_read.read_owned().await;
            #[allow(clippy::unused_io_amount)]
            let result = tokio::task::spawn_blocking(move || {
                let mut data = [0; 256];
                let res = (*lock).pty().read(&mut data);
                (res, data)
            })
            .await
            .unwrap();
            if result.0.is_ok() {
                if send.send(result.1).await.is_err() {
                    break;
                }
            } else {
                quit_send.notify_one();
                break;
            }
        }
    });

    loop {
        tokio::select! {
            Some(data) = recv.recv() => {
                if socket_send
                    .send(Message::binary(data.split(|num| num == &0).next().unwrap()))
                    .await
                    .is_err() {
                        break;
                    }
            }
            data_msg = socket_recv.next() => {
                let lock = cmd_write.read().await;
                let data = if let Some(Ok(data_unwrapped)) = data_msg {
                    data_unwrapped
                } else {
                    let _write = (*cmd_write.read().await)
                        .pty()
                        .write_all("exit\n".as_bytes());
                    continue;
                };
                if data.is_text() && data.to_str().unwrap().get(..4) == Some("size") {
                    let json: TTYSize =
                        DeJson::deserialize_json(&data.to_str().unwrap()[4..]).unwrap();
                    (*lock)
                        .resize_pty(&pty_process::Size::new(json.rows, json.cols))
                        .unwrap();
                } else if (*lock).pty().write_all(data.as_bytes()).is_err() {
                    break;
                }
            }
            _ = quit_recv.notified() => {
                break;
            }
        }
    }

    // Reap PID
    (*cmd.write().await).wait().unwrap();

    log::info!("Closed terminal");
}

#[allow(clippy::too_many_lines)]
pub async fn file_handler(mut socket: warp::ws::WebSocket) {
    let mut req: shared::FileRequest;

    let mut upload_buf = Vec::new();
    let mut upload_max_size = 0;
    let mut upload_current_size = 0;
    let mut upload_path = String::new();
    while let Some(Ok(data)) = socket.next().await {
        if data.is_close() {
            break;
        }
        if data.is_binary() {
            upload_buf.append(&mut data.into_bytes());
            upload_current_size += 1;
            log::debug!(
                "Received {}MB out of {}MB",
                upload_current_size,
                upload_max_size
            );
            if upload_current_size == upload_max_size {
                std::fs::write(&upload_path, &upload_buf).unwrap();
                let _send = socket
                    .send(Message::text(SerJson::serialize_json(
                        &shared::FileUploadFinished { finished: true },
                    )))
                    .await;
            }
            continue;
        }
        let data_str = if let Ok(data_string) = data.to_str() {
            data_string
        } else {
            log::error!("Couldn't convert received data to text");
            continue;
        };
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
                #[allow(
                    clippy::cast_lossless,
                    clippy::cast_sign_loss,
                    clippy::cast_precision_loss,
                    clippy::cast_possible_truncation
                )]
                let size = (buf.len() as f64 / f64::from(1000 * 1000)).ceil() as usize;
                let _send = socket
                    .send(Message::text(SerJson::serialize_json(&shared::FileSize {
                        size,
                    })))
                    .await;
                for i in 0..size {
                    let _send = socket
                        .send(Message::binary(
                            &buf[i * 1000 * 1000..((i + 1) * 1000 * 1000).min(buf.len())],
                        ))
                        .await;
                    log::debug!("Sent {}MB out of {}MB", i, size);
                }
            }
            "up" => {
                upload_path = req.path;
                upload_max_size = req.arg.parse::<usize>().unwrap();
            }
            "save" => std::fs::write(std::path::Path::new(&req.path), &req.arg).unwrap(),
            _ => {}
        }
    }
}
