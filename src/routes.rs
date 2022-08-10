use crate::handle_error;
use crate::shared::CONFIG;
#[cfg(feature = "frontend")]
use crate::DIR;
use anyhow::Context;
use futures::Future;
use hyper::http::{header, HeaderValue};
use hyper::{Body, Method, Request, Response, StatusCode};
use ring::digest;
use tracing::Instrument;

#[cfg(feature = "frontend")]
#[tracing::instrument(skip_all)]
pub fn favicon_route() -> anyhow::Result<Response<Body>> {
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))
        .body(
            handle_error!(
                DIR.get_file("favicon.png").context("Couldn't get favicon"),
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Couldn't get favicon".into())?)
            )
            .contents()
            .into(),
        )?)
}

#[cfg(feature = "frontend")]
#[tracing::instrument(skip_all)]
pub fn assets_route(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let path = req.uri().path().trim_start_matches('/');
    let ext = path.rsplit('.').next().unwrap_or("plain");
    #[allow(unused_mut)]
    // Mute warning, variable is mut because it's used when building for release
    let mut reply = Response::builder()
        .header(
            header::CONTENT_TYPE,
            match ext {
                "js" => HeaderValue::from_static("text/javascript"),
                "svg" => HeaderValue::from_static("image/svg+xml"),
                "png" => HeaderValue::from_static("image/png"),
                _ => HeaderValue::from_str(&format!("text/{}", ext))?,
            },
        )
        .body(
            match DIR.get_file(path) {
                Some(file) => file.contents(),
                None => {
                    tracing::warn!("Couldn't get asset {}", path);
                    return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body("Asset not found".into())?);
                }
            }
            .into(),
        )?;

    #[cfg(all(feature = "frontend", not(debug_assertions)))]
    if ext != "png" {
        reply.headers_mut().insert(
            header::CONTENT_ENCODING,
            header::HeaderValue::from_static("gzip"),
        );
    };

    Ok(reply)
}

#[tracing::instrument(skip_all)]
pub async fn login_route(mut req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let token: String;
    let mut response = Response::new(Body::empty());
    if CONFIG.pass {
        let shasum = hex::encode(
            digest::digest(
                &digest::SHA512,
                &hyper::body::to_bytes(req.body_mut()).await?,
            )
            .as_ref(),
        );
        if shasum == CONFIG.hash {
            let timestamp = jsonwebtoken::get_current_timestamp();

            let fingerprint = match crate::shared::get_fingerprint(&req) {
                Ok(Some(cookie)) => cookie,
                Err(err) => {
                    tracing::warn!("{:#}", err);
                    *response.status_mut() = StatusCode::BAD_REQUEST;
                    *response.body_mut() = "Invalid fingerprint token".into();
                    return Ok(response);
                }
                Ok(None) => {
                    let mut buf = [0u8; 32].to_vec();
                    handle_error!(
                        getrandom::getrandom(&mut buf)
                            .context("Couldn't generate random fingerprint token"),
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body("Couldn't generate random fingerprint token".into())?)
                    );
                    response.headers_mut().insert(
                        hyper::header::SET_COOKIE,
                        hyper::header::HeaderValue::from_str(&format!(
                            "FINGERPRINT={}; Path=/; HttpOnly",
                            hex::encode(&buf)
                        ))
                        .context("Couldn't set fingerprint token")?,
                    );
                    hex::encode(digest::digest(&digest::SHA256, &buf).as_ref())
                }
            };

            let claims = crate::shared::JWTClaims {
                iss: "DietPi Dashboard".to_string(),
                iat: timestamp,
                exp: timestamp + CONFIG.expiry,
                fingerprint,
            };

            token = handle_error!(
                jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &claims,
                    &jsonwebtoken::EncodingKey::from_secret(CONFIG.secret.as_ref()),
                )
                .context("Error creating login token"),
                return Ok({
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    *response.body_mut() = "Couldn't create login token".into();
                    response
                })
            );

            *response.body_mut() = token.into();

            return Ok(response);
        }
        *response.status_mut() = StatusCode::UNAUTHORIZED;
        *response.body_mut() = "Invalid password".into();
        return Ok(response);
    }
    *response.status_mut() = StatusCode::NO_CONTENT;
    *response.body_mut() = "No login needed".into();
    Ok(response)
}

#[cfg(feature = "frontend")]
#[tracing::instrument(skip_all)]
pub fn main_route() -> anyhow::Result<Response<Body>> {
    let file = handle_error!(
        DIR.get_file("index.html")
            .context("Couldn't get main HTML file"),
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Couldn't get main HTML file".into())?)
    )
    .contents();
    let mut reply = Response::new(file.into());
    let headers = reply.headers_mut();

    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        header::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::X_FRAME_OPTIONS,
        header::HeaderValue::from_static("sameorigin"),
    );
    headers.insert("X-Robots-Tag", header::HeaderValue::from_static("none"));
    headers.insert(
        "X-Permitted-Cross-Domain_Policies",
        header::HeaderValue::from_static("none"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        header::HeaderValue::from_static("no-referrer"),
    );
    headers.insert("Content-Security-Policy", header::HeaderValue::from_static("default-src 'self'; style-src 'unsafe-inline' 'self'; connect-src * ws:; object-src 'none';"));
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/html"),
    );
    #[cfg(all(feature = "frontend", not(debug_assertions)))]
    headers.insert(
        header::CONTENT_ENCODING,
        header::HeaderValue::from_static("gzip"),
    );

    Ok(reply)
}

pub fn websocket<F, O>(
    mut req: Request<Body>,
    func: F,
    span: tracing::Span,
    token: String,
) -> anyhow::Result<Response<Body>>
where
    O: Future<Output = ()> + std::marker::Send,
    F: Fn(
            tokio_tungstenite::WebSocketStream<hyper::upgrade::Upgraded>,
            Option<String>,
            String,
        ) -> O
        + std::marker::Send
        + std::marker::Sync
        + 'static,
{
    let key = req
        .headers()
        .get(header::SEC_WEBSOCKET_KEY)
        .context("Failed to read key from headers")?;

    let cookie = if let Ok(cookie) = crate::shared::get_fingerprint(&req) {
        cookie
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())?);
    };

    if req.headers().get(header::CONNECTION) != Some(&HeaderValue::from_static("Upgrade"))
        || req.headers().get(header::UPGRADE) != Some(&HeaderValue::from_static("websocket"))
        || req.headers().get(header::SEC_WEBSOCKET_VERSION) != Some(&HeaderValue::from_static("13"))
    {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())?);
    }

    let resp = Response::builder()
        .status(StatusCode::SWITCHING_PROTOCOLS)
        .header(header::CONNECTION, "upgrade")
        .header(header::UPGRADE, "websocket")
        .header(
            "Sec-WebSocket-Accept",
            &tokio_tungstenite::tungstenite::handshake::derive_accept_key(key.as_bytes()),
        )
        .body(Body::from("switching to websocket protocol"));

    tokio::spawn(async move {
        match hyper::upgrade::on(&mut req).await {
            Ok(upgraded) => {
                let ws = tokio_tungstenite::WebSocketStream::from_raw_socket(
                    upgraded,
                    tokio_tungstenite::tungstenite::protocol::Role::Server,
                    None,
                )
                .await;
                func(ws, cookie, token).instrument(span).await;
            }
            Err(e) => eprintln!("upgrade error: {}", e),
        }
    });
    Ok(resp?)
}

pub async fn router(req: Request<Body>, span: tracing::Span) -> anyhow::Result<Response<Body>> {
    let mut response = Response::new(Body::empty());

    match (
        req.method(),
        req.uri().path().trim_end_matches('/'),
        // Make a String to avoid lifetime errors
        req.uri().query().map(str::to_string),
    ) {
        #[cfg(feature = "frontend")]
        (&Method::GET, "/favicon.png", _) => {
            let _guard = span.enter();
            response = favicon_route()?;
        }
        #[cfg(feature = "frontend")]
        (&Method::GET, path, _) if path.starts_with("/assets") => {
            let _guard = span.enter();
            response = assets_route(req)?;
        }
        (&Method::GET, "/ws", _) => {
            response = websocket(
                req,
                crate::socket_handlers::socket_handler,
                span,
                String::new(),
            )?;
        }
        (&Method::GET, "/ws/term", Some(token)) if crate::CONFIG.pass => {
            let token = crate::shared::get_token_from_list(&token, ['=', ';'], "token");
            if let Some(token) = token {
                response = websocket(
                    req,
                    crate::socket_handlers::term_handler,
                    span,
                    token.to_string(),
                )?;
            } else {
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                *response.body_mut() = "No token".into();
                return Ok(response);
            }
        }
        (&Method::GET, "/ws/file", Some(token)) if crate::CONFIG.pass => {
            let token = crate::shared::get_token_from_list(&token, ['=', ';'], "token");
            if let Some(token) = token {
                response = websocket(
                    req,
                    crate::socket_handlers::file_handler,
                    span,
                    token.to_string(),
                )?;
            } else {
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                *response.body_mut() = "No token".into();
                return Ok(response);
            }
        }
        (&Method::GET, "/ws/term", _) if !crate::CONFIG.pass => {
            response = websocket(
                req,
                crate::socket_handlers::term_handler,
                span,
                String::new(),
            )?;
        }
        (&Method::GET, "/ws/file", _) if !crate::CONFIG.pass => {
            response = websocket(
                req,
                crate::socket_handlers::file_handler,
                span,
                String::new(),
            )?;
        }
        (&Method::POST, "/login", _) => {
            response = login_route(req).instrument(span).await?;
        }
        #[cfg(feature = "frontend")]
        (&Method::GET, _, _) => {
            let _guard = span.enter();
            response = main_route()?;
        }
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
            *response.body_mut() = "Method not allowed".into();
        }
    }

    Ok(response)
}
