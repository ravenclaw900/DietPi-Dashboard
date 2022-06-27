use anyhow::Context;
use futures::{SinkExt, StreamExt};
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
                serde_json::from_str(data_str)
                    .with_context(|| format!("Couldn't parse JSON {}", data_str)),
                continue
            );
            if CONFIG.pass && !validate_token(&req.token) {
                if !first_message {
                    if let Err(err) = data_send.send(None).await {
                        tracing::error!("Internal error: couldn't initiate login: {}", err);
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
                    tracing::error!("Internal error: couldn't change page: {}", err);
                    break;
                }
            }
            // Send new page/data
            if let Err(err) = data_send.send(Some(req.clone())).await {
                // Manual error handling here, to use tracing::error
                tracing::error!("Internal error: couldn't send request: {}", err);
                break;
            }
        }
    });
    // Send global message (shown on all pages)
    if socket_send
        .send(crate::json_msg!(&systemdata::global().await, return))
        .await
        .is_err()
    {
        return;
    }
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
                if socket_send
                    .send(crate::json_msg!(
                        &shared::TokenError { error: true },
                        continue
                    ))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            _ => {
                tracing::debug!("Got page {}, not handling", message.page);
            }
        }
    }
}

#[derive(serde::Deserialize)]
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
                                serde_json::from_str(&data_str[4..]).with_context(|| format!(
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

    tracing::info!("Closed terminal");
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

// Not the most elegant solution, but it works
enum FileHandlerHelperReturns {
    String(String),
    SizeBuf(shared::FileSize, Vec<u8>),
    StreamUpload(usize, tokio::fs::File),
}

async fn file_handler_helper(
    req: &shared::FileRequest,
) -> anyhow::Result<Option<FileHandlerHelperReturns>> {
    match req.cmd.as_str() {
        "open" => {
            return Ok(Some(FileHandlerHelperReturns::String(
                tokio::fs::read_to_string(&req.path)
                    .await
                    .with_context(|| format!("Couldn't read file {}", &req.path))?,
            )))
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
            return Ok(Some(FileHandlerHelperReturns::SizeBuf(
                shared::FileSize { size },
                buf,
            )));
        }
        "up" => {
            let file = tokio::fs::File::create(&req.path)
                .await
                .with_context(|| format!("Couldn't create file at {}", &req.path))?;
            return Ok(Some(FileHandlerHelperReturns::StreamUpload(
                req.arg.parse::<usize>().context("Invalid max size")?,
                file,
            )));
        }
        "save" => tokio::fs::write(&req.path, &req.arg)
            .await
            .with_context(|| format!("Couldn't save file {}", &req.path))?,
        _ => {}
    }
    Ok(None)
}

fn get_file_req(data: &warp::ws::Message) -> anyhow::Result<shared::FileRequest> {
    let data_str = data
        .to_str()
        .map_err(|_| anyhow::anyhow!("Couldn't convert received data {:?} to text", data))?;
    let req = serde_json::from_str(data_str)
        .with_context(|| format!("Couldn't parse JSON from {}", data_str))?;
    Ok(req)
}

pub async fn file_handler(socket: warp::ws::WebSocket) {
    let (mut socket_send, mut socket_recv) = socket.split();
    let mut req: shared::FileRequest;

    'outer: while let Some(Ok(data)) = socket_recv.next().await {
        if data.is_close() {
            break;
        }

        req = handle_error!(get_file_req(&data), continue);

        if CONFIG.pass && !validate_token(&req.token) {
            continue;
        }

        loop {
            tokio::select! {
                result = file_handler_helper(&req) => {
                    match handle_error!(result, continue) {
                        Some(FileHandlerHelperReturns::String(file)) => {
                            if socket_send.send(Message::text(file)).await.is_err() {
                                break 'outer;
                            }
                        }
                        Some(FileHandlerHelperReturns::SizeBuf(size, buf)) => {
                            if socket_send
                                .send(crate::json_msg!(&size, continue))
                                .await
                                .is_err()
                            {
                                break 'outer;
                            }
                            for i in buf.chunks(1000 * 1000) {
                                if socket_send.feed(Message::binary(i)).await.is_err() {
                                    break 'outer;
                                }
                            }
                            if socket_send.flush().await.is_err() {
                                break 'outer;
                            }
                        }
                        Some(FileHandlerHelperReturns::StreamUpload(size, mut file)) => {
                            while let Some(Ok(msg)) = (&mut socket_recv).take(size).next().await {
                                handle_error!(file.write_all(msg.as_bytes()).await.with_context(|| {
                                    format!("Couldn't write to file {}, stopping upload", &req.path)
                                }), continue 'outer);
                            }
                        }
                        None => {}
                    }
                    break;
                },
                recv = socket_recv.next() => match recv {
                    Some(Ok(req_tmp)) => req = handle_error!(get_file_req(&req_tmp), continue 'outer),
                    _ => break 'outer,
                },
            }
        }
    }
}
