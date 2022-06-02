#![warn(clippy::pedantic)]
use crate::shared::CONFIG;
use anyhow::Context;
use ring::digest;
use simple_logger::SimpleLogger;
use std::net::IpAddr;
#[cfg(feature = "frontend")]
use warp::http::header;
use warp::Filter;

mod config;
mod page_handlers;
mod shared;
mod socket_handlers;
mod systemdata;

#[allow(clippy::too_many_lines)]
fn main() -> anyhow::Result<()> {
    #[allow(clippy::cast_possible_truncation)]
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(psutil::cpu::cpu_count().max(2) as usize)
        .enable_all()
        .build()
        .context("Couldn't start tokio runtime")?
        .block_on(async {
            #[cfg(feature = "frontend")]
            const DIR: include_dir::Dir = include_dir::include_dir!("dist");

            SimpleLogger::new()
                .with_level(CONFIG.log_level)
                .env()
                .init()
                .context("Couldn't init logger")?;

            #[cfg(feature = "frontend")]
            let mut headers = header::HeaderMap::new();
            #[cfg(feature = "frontend")]
            {
            headers.insert(header::X_CONTENT_TYPE_OPTIONS, header::HeaderValue::from_static("nosniff"));
            headers.insert(header::X_FRAME_OPTIONS, header::HeaderValue::from_static("sameorigin"));
            headers.insert("X-Robots-Tag", header::HeaderValue::from_static("none"));
            headers.insert("X-Permitted-Cross-Domain_Policies", header::HeaderValue::from_static("none"));
            headers.insert(header::REFERRER_POLICY, header::HeaderValue::from_static("no-referrer"));
            headers.insert("Content-Security-Policy", header::HeaderValue::from_static("default-src 'self'; font-src 'self'; img-src 'self' blob:; script-src 'self'; style-src 'unsafe-inline' 'self'; connect-src * ws:;"));
            }

            #[cfg(feature = "frontend")]
            let favicon_route = warp::path("favicon.png").map(|| {
                warp::reply::with_header(
                    // Assume that favicon will always work (if it doesn't, there's probably a bigger problem)
                    DIR.get_file("favicon.png").expect("Couldn't get favicon").contents(),
                    "content-type",
                    "image/png",
                )
            });

            #[cfg(feature = "frontend")]
            let assets_route = warp::path("assets")
                .and(warp::path::param())
                .map(|path: String| {
                    let ext = path.rsplit('.').next().unwrap();
                    warp::reply::with_header(
                        match DIR.get_file(format!("assets/{}", path)) {
                            Some(file) => file.contents(),
                            None => {
                                log::warn!("Couldn't get asset {}", path);
                                &[]
                            }
                        },
                        "content-type",
                        if ext == "js" {
                            "text/javascript".to_string()
                        } else if ext == "svg" {
                            "image/svg+xml".to_string()
                        } else {
                            format!("text/{}", ext)
                        },
                    )
                });

            let login_route = warp::path("login")
                .and(warp::post())
                .and(warp::body::bytes())
                .map(|pass: warp::hyper::body::Bytes| {
                    if CONFIG.pass {
                        let shasum = digest::digest(&digest::SHA512, &pass).as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
                        if shasum == CONFIG.hash {
                            let timestamp = jsonwebtoken::get_current_timestamp();

                            let claims = crate::shared::JWTClaims {
                                iss: "DietPi Dashboard".to_string(),
                                iat: timestamp,
                                exp: timestamp + CONFIG.expiry,
                            };

                            let token = jsonwebtoken::encode(
                                &jsonwebtoken::Header::default(),
                                &claims,
                                &jsonwebtoken::EncodingKey::from_secret(CONFIG.secret.as_ref()),
                            )
                            .expect("Error creating login token");

                            return warp::reply::with_status(token, warp::http::StatusCode::OK);
                        }
                        return warp::reply::with_status(
                            "Unauthorized".to_string(),
                            warp::http::StatusCode::UNAUTHORIZED,
                        );
                    }
                    warp::reply::with_status(
                        "No login needed".to_string(),
                        warp::http::StatusCode::OK,
                    )
                })
                .with(warp::reply::with::header(
                    "Access-Control-Allow-Origin",
                    "*",
                ));

            let terminal_route = warp::path("ws")
                .and(warp::path("term"))
                .and(warp::ws())
                .map(|ws: warp::ws::Ws| ws.on_upgrade(socket_handlers::term_handler));

            let socket_route = warp::path("ws")
                .and(warp::ws())
                .map(|ws: warp::ws::Ws| ws.on_upgrade(socket_handlers::socket_handler));

            let file_route = warp::path("ws")
                .and(warp::path("file"))
                .and(warp::ws())
                .map(|ws: warp::ws::Ws| ws.on_upgrade(socket_handlers::file_handler));

            #[cfg(feature = "frontend")]
            let main_route = warp::any().map(|| {
                warp::reply::html(DIR.get_file("index.html").expect("Couldn't get main HTML file").contents_utf8().expect("Invalid main HTML file"))
            }).with(warp::reply::with::headers(headers));

            #[cfg(feature = "frontend")]
            let page_routes = favicon_route
                .or(assets_route)
                .or(main_route)
                .with(warp::compression::gzip());

            let socket_routes = terminal_route.or(file_route).or(socket_route);

            let routes = socket_routes
                .or(login_route)
                .with(warp::log::custom(|info| {
                    log::info!("Request to {}", info.path());
                    log::debug!(
                        "by {}, using {} {:?}, with response of HTTP code {:?}",
                        info.remote_addr().unwrap().ip(),
                        info.user_agent().unwrap(),
                        info.version(),
                        info.status()
                    );
                }));

            #[cfg(feature = "frontend")]
            let routes = routes.or(page_routes);

            let addr = IpAddr::from([0; 8]);

            if CONFIG.tls {
                warp::serve(routes)
                    .tls()
                    .cert_path(&CONFIG.cert)
                    .key_path(&CONFIG.key)
                    .run((addr, CONFIG.port))
                    .await;
            } else {
                warp::serve(routes).run((addr, CONFIG.port)).await;
            }

            anyhow::Ok(())
        })?;

    Ok(())
}
