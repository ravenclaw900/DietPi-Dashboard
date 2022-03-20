#![warn(clippy::pedantic)]
use crate::shared::CONFIG;
use sha2::{Digest, Sha512};
use simple_logger::SimpleLogger;
use warp::Filter;

mod config;
mod page_handlers;
mod shared;
mod socket_handlers;
mod systemdata;

#[allow(clippy::too_many_lines)]
fn main() {
    #[allow(clippy::cast_possible_truncation)]
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(psutil::cpu::cpu_count().max(2) as usize)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            #[cfg(feature = "frontend")]
            const DIR: include_dir::Dir = include_dir::include_dir!("dist");

            SimpleLogger::new()
                .with_level(CONFIG.log_level)
                .env()
                .init()
                .unwrap();

            #[cfg(feature = "frontend")]
            let favicon_route = warp::path("favicon.png").map(|| {
                warp::reply::with_header(
                    DIR.get_file("favicon.png").unwrap().contents(),
                    "content-type",
                    "image/png",
                )
            });

            #[cfg(feature = "frontend")]
            let assets_route = warp::path("assets")
                .and(warp::path::param())
                .map(|path: String| {
                    warp::reply::with_header(
                        DIR.get_file(format!("assets/{}", path)).unwrap().contents(),
                        "content-type",
                        format!(
                            "text/{}",
                            if path.rsplit('.').next().unwrap() == "js" {
                                "javascript"
                            } else {
                                path.rsplit('.').next().unwrap()
                            }
                        ),
                    )
                });

            let login_route = warp::path("login")
                .and(warp::post())
                .and(warp::body::bytes())
                .map(|pass| {
                    if CONFIG.pass {
                        let mut hasher = Sha512::new();
                        hasher.update(pass);
                        let shasum = format!("{:x}", hasher.finalize());
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
                warp::reply::html(DIR.get_file("index.html").unwrap().contents_utf8().unwrap())
            });

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

            if CONFIG.tls {
                warp::serve(routes)
                    .tls()
                    .cert_path(&CONFIG.cert)
                    .key_path(&CONFIG.key)
                    .run(([0, 0, 0, 0], CONFIG.port))
                    .await;
            } else {
                warp::serve(routes).run(([0, 0, 0, 0], CONFIG.port)).await;
            }
        });
}
