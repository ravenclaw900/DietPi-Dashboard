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
    validator.set_required_spec_claims(&["exp", "iat"]);
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
            req = match DeJson::deserialize_json(data_str) {
                Ok(json) => json,
                Err(err) => {
                    log::error!("Couldn't parse JSON: {}", err);
                    continue;
                }
            };
            if CONFIG.pass && !validate_token(&req.token) {
                quit_send.notify_waiters();
                if let Err(err) = data_send
                    .send(shared::Request {
                        page: "/login".to_string(),
                        token: String::new(),
                        cmd: String::new(),
                        args: Vec::new(),
                    })
                    .await
                {
                    log::error!("Internal error: couldn't request login: {}", err);
                }
                continue;
            }
            if req.cmd.is_empty() {
                // Quit out of handler
                quit_send.notify_waiters();
            }
            // Send new page/data
            if let Err(err) = data_send.send(req.clone()).await {
                log::error!("Internal error: couldn't send page data: {}", err);
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

#[allow(clippy::too_many_lines)]
pub async fn term_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();

    if crate::CONFIG.pass {
        if let Some(Ok(token)) = socket_recv.next().await {
            // Stop from panicking, return from function with invalid token instead
            let token = token.to_str().unwrap_or("");
            if token.get(..5) == Some("token") {
                if !validate_token(&token[5..]) {
                    return;
                }
            } else {
                return;
            }
        } else {
            return;
        }
    }

    let mut pre_cmd = std::process::Command::new("/bin/login");
    let mut pre_cmd = pre_cmd.env("TERM", "xterm");
    if crate::CONFIG.terminal_user != "manual" {
        pre_cmd = pre_cmd.args(&["-f", &crate::CONFIG.terminal_user]);
    }

    let cmd = Arc::new(RwLock::new(match pre_cmd.spawn_pty(None) {
        Ok(pre_cmd) => pre_cmd,
        Err(err) => {
            log::warn!("Error creating pty: {}", err);
            return;
        }
    }));
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
            if let Ok(result) = tokio::task::spawn_blocking(move || {
                let mut data = [0; 256];
                let res = (*lock).pty().read(&mut data);
                (res, data)
            })
            .await
            {
                if result.0.is_ok() {
                    if let Err(err) = send.send(result.1).await {
                        log::warn!("Internal error: couldn't send data between ptys: {}", err);
                        break;
                    }
                } else {
                    // Generally this means that the pty has closed (i.e. page changed), so no error message
                    quit_send.notify_one();
                    break;
                }
            } else {
                log::warn!("Internal error: couldn't start thread to read from pty");
                break;
            }
        }
    });

    loop {
        tokio::select! {
            Some(data) = recv.recv() => {
                // No error message because socket could have just closed
                if socket_send
                    .send(Message::binary(data.split(|num| num == &0).next().unwrap())) // Ignore <256 '0' bytes (should be guaranteed to not fail)
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
            // For sure a text message, so should unwrap correctly
            if data.is_text() && data.to_str().unwrap().get(..4) == Some("size") {
                let json: TTYSize = match DeJson::deserialize_json(&data.to_str().unwrap()[4..]) {
                    Ok(size) => size,
                    Err(err) => {
                        log::warn!("Couldn't deserialize terminal size: {}", err);
                        continue;
                    }
                };
                if let Err(err) = (*lock).resize_pty(&pty_process::Size::new(json.rows, json.cols)) {
                    log::warn!("Couldn't resize terminal: {}", err);
                    continue;
                }
                } else if (*lock).pty().write_all(data.as_bytes()).is_err() {
                    // No error message because pty could have been closed by us
                    break;
                }
            }
            _ = quit_recv.notified() => {
                break;
            }
        }
    }

    let reap = &mut *cmd.write().await;
    // Reap PID
    if let Err(err) = reap.wait() {
        log::warn!("Couldn't close terminal: {}", err);
    } else {
        log::info!("Closed terminal");
    }
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
                if let Err(err) = std::fs::write(&upload_path, &upload_buf) {
                    log::warn!("Couldn't write uploaded file: {}", err);
                }
                let _send = socket
                    .send(Message::text(SerJson::serialize_json(
                        &shared::FileUploadFinished { finished: true },
                    )))
                    .await;
            }
            continue;
        }
        let data_str = if let Ok(data_str) = data.to_str() {
            data_str
        } else {
            log::error!("Couldn't convert received data to text");
            continue;
        };
        req = match DeJson::deserialize_json(data_str) {
            Ok(json) => json,
            Err(err) => {
                log::error!("Couldn't parse JSON: {}", err);
                continue;
            }
        };
        if CONFIG.pass && !validate_token(&req.token) {
            continue;
        }

        match req.cmd.as_str() {
            "open" => {
                let data = match std::fs::read_to_string(&req.path) {
                    Ok(data) => data,
                    Err(err) => {
                        log::warn!("Couldn't read file {}: {}", &req.path, err);
                        continue;
                    }
                };
                let _send = socket.send(Message::text(data)).await;
            }
            // Technically works for both files and directories
            "dl" => {
                let mut buf = Vec::new();
                let src_path = std::path::Path::new(&req.path);
                if !src_path.exists() {
                    log::warn!("Path doesn't exist");
                    continue;
                }
                {
                    let mut zip_file = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
                    let mut file_buf = Vec::new();
                    for entry in walkdir::WalkDir::new(&req.path) {
                        let entry = match entry {
                            Ok(entry) => entry,
                            Err(err) => {
                                log::warn!("Couldn't recursively get directory: {}", err);
                                continue;
                            }
                        };
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
            "save" => std::fs::write(&req.path, &req.arg).unwrap(),
            _ => {}
        }
    }
}
