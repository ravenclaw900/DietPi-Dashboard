use crate::handle_error;
use crate::shared::CONFIG;
use anyhow::Context;
use bytes::Bytes;
use futures_util::Future;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::header;
use hyper::{body::Incoming, Error, Method, Request, Response, StatusCode};
use ring::digest;
use tracing::Instrument;

#[cfg(feature = "dev")]
vite_embed::generate_vite_html_dev!("$CARGO_MANIFEST_DIR/frontend/index.html", "src/main.ts");

#[cfg(feature = "dev")]
async fn frontend_proxy(path: &str) -> anyhow::Result<Response<BoxBody<Bytes, Error>>> {
    let path = path.to_string();
    let vite_resp = tokio::task::spawn_blocking(move || (vite_embed::vite_proxy_dev(&path), path))
        .await
        .expect("Failed to spawn blocking call to frontend");

    match vite_resp {
        (Ok(body), _) => Ok(Response::new(full(body))),
        (Err(vite_embed::RequestError::Status(code, _)), _) => {
            Ok(Response::builder().status(code).body(empty())?)
        }
        (_, path) => Err(anyhow::anyhow!(
            "Failed to connect to dev server while getting {}",
            path
        )),
    }
}

#[cfg(all(feature = "frontend", not(feature = "dev")))]
vite_embed::generate_vite_prod!(
    "$CARGO_MANIFEST_DIR/frontend/dist/manifest.json",
    "src/main.ts",
    "$CARGO_MANIFEST_DIR/frontend/index.html"
);

#[cfg(all(feature = "frontend", not(feature = "dev")))]
fn main_route() -> Response<BoxBody<Bytes, Error>> {
    // index.html is guaranteed to exist, compilation would fail if it didn't
    #[allow(clippy::unwrap_used)]
    let mut reply = Response::new(full(vite_prod("/index.html").unwrap().0));
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
        "X-Permitted-Cross-Domain-Policies",
        header::HeaderValue::from_static("none"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        header::HeaderValue::from_static("no-referrer"),
    );
    headers.insert("Content-Security-Policy", header::HeaderValue::from_static("default-src 'self'; style-src 'unsafe-inline' 'self'; connect-src * ws:; object-src 'none'; img-src 'self' data:;"));
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/html"),
    );
    headers.insert(
        header::CONTENT_ENCODING,
        header::HeaderValue::from_static("gzip"),
    );

    reply
}

#[cfg(all(feature = "frontend", not(feature = "dev")))]
fn asset_route(path: &str) -> Response<BoxBody<Bytes, Error>> {
    let mut reply = Response::new(empty());
    let Some((data, compressed)) = vite_prod(path) else {
        *reply.status_mut() = StatusCode::NOT_FOUND;
        *reply.body_mut() = full("Asset not found");
        return reply;
    };

    if compressed {
        reply.headers_mut().insert(
            header::CONTENT_ENCODING,
            header::HeaderValue::from_static("gzip"),
        );
    }

    let mime_type = match path.rsplit_once('.').unwrap_or(("plain", "plain")).1 {
        "js" => header::HeaderValue::from_static("text/javascript"),
        "svg" => header::HeaderValue::from_static("image/svg+xml"),
        "png" => header::HeaderValue::from_static("image/png"),
        "css" => header::HeaderValue::from_static("text/css"),
        // There should be no other file types
        _ => unreachable!(),
    };

    reply.headers_mut().insert(header::CONTENT_TYPE, mime_type);

    *reply.body_mut() = full(data);

    reply
}

#[tracing::instrument(skip_all)]
pub async fn login_route(
    req: Request<Incoming>,
) -> anyhow::Result<Response<BoxBody<Bytes, Error>>> {
    let token: String;
    let mut response = Response::new(empty());
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        header::HeaderValue::from_static("*"),
    );
    if CONFIG.pass {
        let shasum =
            hex::encode(digest::digest(&digest::SHA512, &req.collect().await?.to_bytes()).as_ref());
        if shasum == CONFIG.hash {
            let timestamp = jsonwebtoken::get_current_timestamp();

            let fingerprint = match crate::shared::get_fingerprint(&req) {
                Ok(Some(cookie)) => cookie,
                Err(err) => {
                    tracing::warn!("{:#}", err);
                    *response.status_mut() = StatusCode::BAD_REQUEST;
                    *response.body_mut() = full("Invalid fingerprint token");
                    return Ok(response);
                }
                Ok(None) => {
                    let mut buf = [0u8; 32].to_vec();
                    handle_error!(
                        getrandom::getrandom(&mut buf)
                            .context("Couldn't generate random fingerprint token"),
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(full("Couldn't generate random fingerprint token"))?)
                    );
                    response.headers_mut().insert(
                        header::SET_COOKIE,
                        header::HeaderValue::from_str(&format!(
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
                    *response.body_mut() = full("Couldn't create login token");
                    response
                })
            );

            *response.body_mut() = full(token);

            return Ok(response);
        }
        *response.status_mut() = StatusCode::UNAUTHORIZED;
        *response.body_mut() = full("Invalid password");
        return Ok(response);
    }
    *response.status_mut() = StatusCode::NO_CONTENT;
    *response.body_mut() = full("No login needed");
    Ok(response)
}

pub fn websocket<F, O>(
    mut req: Request<Incoming>,
    func: F,
    span: tracing::Span,
    token: String,
) -> anyhow::Result<Response<BoxBody<Bytes, Error>>>
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

    let Ok(cookie) = crate::shared::get_fingerprint(&req) else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(empty())?);
    };

    if !(req
        .headers()
        .get(header::CONNECTION)
        .and_then(|x| x.to_str().ok())
        .map_or(false, |x| x.contains("Upgrade"))
        && req
            .headers()
            .get(header::UPGRADE)
            .and_then(|x| x.to_str().ok())
            .map_or(false, |x| x.contains("websocket"))
        && req.headers().get(header::SEC_WEBSOCKET_VERSION)
            == Some(&header::HeaderValue::from_static("13")))
    {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(empty())?);
    }

    let resp = Response::builder()
        .status(StatusCode::SWITCHING_PROTOCOLS)
        .header(header::CONNECTION, "upgrade")
        .header(header::UPGRADE, "websocket")
        .header(
            "Sec-WebSocket-Accept",
            &tokio_tungstenite::tungstenite::handshake::derive_accept_key(key.as_bytes()),
        )
        .body(full("switching to websocket protocol"));

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
            Err(e) => eprintln!("upgrade error: {e}"),
        }
    });
    Ok(resp?)
}

pub async fn router(
    req: Request<Incoming>,
    span: tracing::Span,
) -> anyhow::Result<Response<BoxBody<Bytes, Error>>> {
    let mut response = Response::new(empty());

    let mut path = req.uri().path();
    if path != "/" {
        path = path.trim_end_matches('/');
    }

    match (
        req.method(),
        path,
        // Make a String to avoid lifetime errors
        req.uri().query().map(str::to_string),
    ) {
        #[cfg(all(feature = "frontend", not(feature = "dev")))]
        (&Method::GET, "/favicon.png", _) => {
            let _guard = span.enter();
            response = asset_route("/favicon.png");
        }
        #[cfg(all(feature = "frontend", not(feature = "dev")))]
        (&Method::GET, path, _) if path.starts_with("/assets") => {
            let _guard = span.enter();
            response = asset_route(req.uri().path());
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
                *response.body_mut() = full("No token");
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
                *response.body_mut() = full("No token");
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
        #[cfg(feature = "dev")]
        (&Method::GET, "/", _) => {
            let _guard = span.enter();
            response = Response::new(full(vite_html_dev()));
        }
        #[cfg(feature = "dev")]
        (&Method::GET | &Method::POST, _, _) => {
            let _guard = span.enter();
            response = frontend_proxy(req.uri().path()).await?;
        }
        #[cfg(all(feature = "frontend", not(feature = "dev")))]
        (&Method::GET, _, _) => {
            let _guard = span.enter();
            response = main_route();
        }
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
            *response.body_mut() = full("Method not allowed");
        }
    }

    Ok(response)
}

fn empty() -> BoxBody<Bytes, Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
