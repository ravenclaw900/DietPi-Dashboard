#![warn(clippy::pedantic)]
use crate::shared::CONFIG;
use sha2::{Digest, Sha512};
use simple_logger::SimpleLogger;
use warp::Filter;

mod config;
mod shared;
mod sockets;
mod systemdata;
mod terminal;

#[allow(clippy::too_many_lines)]
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get().max(2)) // We have to use num_cpus because heim is async, and the runtime hasn't been started yet. Minimum of 2 threads.
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            const DIR: include_dir::Dir = include_dir::include_dir!("dist");

            SimpleLogger::new()
                .with_level(log::LevelFilter::Info)
                .env()
                .init()
                .unwrap();

            let favicon_route = warp::path("favicon.png").map(|| {
                warp::reply::with_header(
                    DIR.get_file("favicon.png").unwrap().contents(),
                    "content-type",
                    "image/png",
                )
            });

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
                            let mut claims = jwts::Claims::new();
                            claims.exp = Some(
                                (std::time::SystemTime::now()
                                    + std::time::Duration::from_secs(3600))
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                            );
                            claims.iss = Some("DietPi Dashboard".to_string());
                            let mut token = jwts::jws::Token::with_payload(claims);
                            let key =
                                jwts::jws::Key::new(&CONFIG.secret, jwts::jws::Algorithm::HS256);
                            return warp::reply::with_status(
                                token.sign(&key).unwrap(),
                                warp::http::StatusCode::OK,
                            );
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
                });

            let terminal_route = warp::path!("ws" / "term")
                .and(warp::ws())
                .map(|ws: warp::ws::Ws| ws.on_upgrade(terminal::term_handler));

            let socket_route = warp::path("ws")
                .and(warp::ws())
                .map(|ws: warp::ws::Ws| ws.on_upgrade(sockets::socket_handler));

            let main_route = warp::any().map(|| {
                warp::reply::html(DIR.get_file("index.html").unwrap().contents_utf8().unwrap())
            });

            let page_routes = favicon_route
                .or(assets_route)
                .or(login_route)
                .or(main_route)
                .with(warp::compression::gzip());

            let socket_routes = terminal_route.or(socket_route);

            let routes = socket_routes
                .or(page_routes)
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
