use anyhow::Context;
use futures::{SinkExt, StreamExt};
use nanoserde::{DeJson, SerJson};
use pty_process::Command;
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use warp::ws::Message;

use crate::{handle_error, page_handlers, shared, systemdata, CONFIG};

fn validate_token(token: &str) -> bool {
    let mut validator = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validator.set_issuer(&["DietPi Dashboard"]);
    validator.set_required_spec_claims(&["exp", "iat"]);
    if jsonwebtoken::decode::<shared::JWTClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(CONFIG.secret.as_bytes()),
        &validator,
    )
    .is_err()
    {
        return false;
    }
    true
}

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
            let data_str = handle_error!(
                data.to_str().map_err(|_| anyhow::anyhow!(
                    "Couldn't convert received data {:?} to text",
                    data
                )),
                continue
            );
            req = handle_error!(
                DeJson::deserialize_json(data_str)
                    .with_context(|| format!("Couldn't parse JSON {}", data_str)),
                continue
            );
            if CONFIG.pass && !validate_token(&req.token) {
                if !first_message {
                    if let Err(err) = data_send.send(None).await {
                        log::error!("Internal error: couldn't initiate login: {}", err);
                        break;
                    }
                }
                handle_error!(data_send
                    .send(Some(shared::Request {
                        page: "/login".to_string(),
                        token: String::new(),
                        cmd: String::new(),
                        args: Vec::new(),
                    }))
                    .await
                    .context("Internal error: couldn't send login request"));
                continue;
            }
            if req.cmd.is_empty() {
                // Quit out of handler
                if first_message {
                    first_message = false;
                } else if let Err(err) = data_send.send(None).await {
                    log::error!("Internal error: couldn't change page: {}", err);
                    break;
                }
            }
            // Send new page/data
            if let Err(err) = data_send.send(Some(req.clone())).await {
                // Manual error handling here, to use log::error
                log::error!("Internal error: couldn't send request: {}", err);
                break;
            }
        }
    });
    // Send global message (shown on all pages)
    let _send = socket_send
        .send(Message::text(SerJson::serialize_json(
            &systemdata::global().await,
        )))
        .await;
    while let Some(Some(message)) = data_recv.recv().await {
        match message.page.as_str() {
            "/" => page_handlers::main_handler(&mut socket_send, &mut data_recv).await,
            "/process" => {
                page_handlers::process_handler(&mut socket_send, &mut data_recv).await;
            }
            "/software" => {
                page_handlers::software_handler(&mut socket_send, &mut data_recv).await;
            }
            "/management" => {
                page_handlers::management_handler(&mut socket_send, &mut data_recv).await;
            }
            "/service" => {
                page_handlers::service_handler(&mut socket_send, &mut data_recv).await;
            }
            "/browser" => {
                page_handlers::browser_handler(&mut socket_send, &mut data_recv).await;
            }
            "/login" => {
                // Internal poll, see other thread
                let _send = socket_send
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
        }
    }

    // We use std::process::Command here because it lets us read and write without a mutable reference (even though it should require one?)
    let mut pre_cmd = std::process::Command::new("/bin/login");
    pre_cmd.env("TERM", "xterm");

    let mut cmd = Arc::new(handle_error!(
        if crate::CONFIG.terminal_user == "manual" {
            &mut pre_cmd
        } else {
            pre_cmd.args(&["-f", &crate::CONFIG.terminal_user])
        }
        .spawn_pty(None)
        .context("Couldn't spawn pty"),
        return
    ));
    let mut cmd_read = Arc::clone(&cmd);

    tokio::join!(
        async {
            loop {
                // Don't care about partial reads, it's in a loop
                #[allow(clippy::unused_io_amount)]
                let result = handle_error!(
                    tokio::task::spawn_blocking(move || {
                        let mut data = [0; 256];
                        let res = cmd_read.pty().read(&mut data);
                        (res, data, cmd_read)
                    })
                    .await
                    .context("Couldn't spawn tokio reader thread"),
                    break
                );
                cmd_read = result.2;
                if result.0.is_ok() {
                    if socket_send
                        .send(Message::binary(
                            result.1.split(|num| *num == 0).next().unwrap_or(&result.1),
                        )) // Should never be None, but return data just in case
                        .await
                        .is_err()
                    {
                        break;
                    }
                } else {
                    break;
                }
            }
        },
        async {
            loop {
                match socket_recv.next().await {
                    Some(Ok(data)) => {
                        if data.is_text() && data.to_str().unwrap().get(..4) == Some("size") {
                            let data_str = data.to_str().unwrap();
                            let json: TTYSize = handle_error!(
                                DeJson::deserialize_json(&data_str[4..]).with_context(|| format!(
                                    "Couldn't deserialize pty size from {}",
                                    &data_str
                                )),
                                continue
                            );
                            handle_error!(cmd
                                .resize_pty(&pty_process::Size::new(json.rows, json.cols))
                                .context("Couldn't resize pty"));
                        } else if cmd.pty().write_all(data.as_bytes()).is_err() {
                            break;
                        }
                    }
                    None | Some(Err(_)) => {
                        // Stop bash by writing "exit", since it won't respond to a SIGTERM
                        let _write = cmd.pty().write_all("exit\n".as_bytes());
                        break;
                    }
                }
            }
        }
    );

    // Reap PID, unwrap is safe because all references will have been dropped
    handle_error!(
        Arc::get_mut(&mut cmd)
            .unwrap()
            .wait()
            .context("Couldn't close terminal"),
        return
    );

    log::info!("Closed terminal");
}

async fn create_zip_file(req: &shared::FileRequest) -> anyhow::Result<Vec<u8>> {
    let src_path = tokio::fs::canonicalize(&req.path)
        .await
        .with_context(|| format!("Invalid source path {}", &req.path))?;
    if src_path.is_dir() {
        let mut zip_file = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        for entry in walkdir::WalkDir::new(&src_path) {
            let entry = entry.context("Couldn't get data for recursive entry")?;
            let path = entry.path();
            // 'unwrap' is safe here because path is canonicalized
            let name = std::path::Path::new(&src_path.file_name().unwrap()).join(
                // Here too, because the path should always be a child
                path.strip_prefix(&src_path).unwrap(),
            );
            let name = name.to_string_lossy().to_string();
            if path.is_file() {
                let mut file_buf = Vec::new();
                zip_file
                    .start_file(&name, zip::write::FileOptions::default())
                    .with_context(|| format!("Couldn't add file {} to zip", &name))?;
                let mut file = handle_error!(
                    tokio::fs::File::open(path)
                        .await
                        .with_context(|| format!("Couldn't open file {}, skipping", &name)),
                    continue
                );
                handle_error!(
                    file.read_to_end(&mut file_buf)
                        .await
                        .with_context(|| format!("Couldn't read file {}, skipping", &name)),
                    continue
                );
                let tup = tokio::task::spawn_blocking(
                    move || -> (zip::ZipWriter<std::io::Cursor<Vec<u8>>>, Vec<u8>, anyhow::Result<()>) {
                        if let Err(err) = zip_file.write_all(&file_buf) {
                            return (zip_file, file_buf, Err(err.into()));
                        }
                        (zip_file, file_buf, Ok(()))
                    },
                )
                .await
                .context("Couldn't spawn zip task")?;
                zip_file = tup.0;
                file_buf = tup.1;
                handle_error!(tup
                    .2
                    .with_context(|| format!("Couldn't write file {} into zip, skipping", name)));
                file_buf.clear();
            } else if !name.is_empty() {
                zip_file
                    .add_directory(&name, zip::write::FileOptions::default())
                    .with_context(|| format!("Couldn't add directory {} to zip", name))?;
            }
        }
        return Ok(zip_file
            .finish()
            .context("Couldn't finish writing to zip file")?
            .into_inner());
    }
    tokio::fs::read(&src_path)
        .await
        .with_context(|| format!("Couldn't read file {}", src_path.display()))
}

async fn file_handler_helper(
    req: &shared::FileRequest,
    socket: &mut warp::ws::WebSocket,
) -> anyhow::Result<()> {
    match req.cmd.as_str() {
        "open" => {
            let _send = socket
                .send(Message::text(
                    tokio::fs::read_to_string(&req.path)
                        .await
                        .with_context(|| format!("Couldn't read file {}", &req.path))?,
                ))
                .await;
        }
        // Technically works for both files and directories
        "dl" => {
            let buf = create_zip_file(req).await?;
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
            for i in buf.chunks(1000 * 1000) {
                let _feed = socket.feed(Message::binary(i)).await;
            }
            let _send = socket.flush().await;
        }
        "up" => {
            let mut file = tokio::fs::File::create(&req.path)
                .await
                .with_context(|| format!("Couldn't create file at {}", &req.path))?;
            while let Some(Ok(msg)) = socket
                .take(req.arg.parse::<usize>().context("Invalid max size")?)
                .next()
                .await
            {
                file.write_all(msg.as_bytes()).await.with_context(|| {
                    format!("Couldn't write to file at {}, stopping upload", &req.path)
                })?;
            }
        }
        "save" => tokio::fs::write(&req.path, &req.arg)
            .await
            .with_context(|| format!("Couldn't save file {}", &req.path))?,
        _ => {}
    }
    Ok(())
}

pub async fn file_handler(mut socket: warp::ws::WebSocket) {
    let mut req: shared::FileRequest;

    while let Some(Ok(data)) = socket.next().await {
        if data.is_close() {
            break;
        }

        let data_str = handle_error!(
            data.to_str()
                .map_err(|_| anyhow::anyhow!("Couldn't convert received data {:?} to text", data)),
            continue
        );
        req = handle_error!(
            DeJson::deserialize_json(data_str)
                .with_context(|| format!("Couldn't parse JSON from {}", data_str)),
            continue
        );
        if CONFIG.pass && !validate_token(&req.token) {
            continue;
        }
        handle_error!(file_handler_helper(&req, &mut socket).await, continue);
    }
}
