#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
use crate::shared::CONFIG;
use anyhow::Context;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Response, StatusCode};
use std::{net::IpAddr, str::FromStr};
//use tracing_subscriber::layer::{Layer, SubscriberExt};

mod config;
mod page_handlers;
mod routes;
mod shared;
mod socket_handlers;
mod systemdata;

/*struct BeQuietWarp {
    log_level: tracing_subscriber::filter::LevelFilter,
}

impl<S: tracing::Subscriber> Layer<S> for BeQuietWarp {
    fn enabled(
        &self,
        metadata: &tracing::Metadata<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        !(metadata.target() == "warp::filters::trace" && *metadata.level() >= self.log_level)
    }
}*/

#[cfg(feature = "frontend")]
const DIR: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    {
        let log_level = tracing_subscriber::filter::LevelFilter::from_str(&CONFIG.log_level)
            .context("Couldn't parse log level")?;
        tracing::subscriber::set_global_default(
            tracing_subscriber::FmtSubscriber::builder()
                .with_max_level(log_level)
                .with_timer(tracing_subscriber::fmt::time::uptime())
                .finish(), //.with(BeQuietWarp { log_level }),
        )
        .context("Couldn't init logger")?;
    }

    /*

    #[cfg(feature = "frontend")]
    let page_routes = favicon_route.or(assets_route).or(main_route);

    let socket_routes = terminal_route.or(file_route).or(socket_route);

    let routes = socket_routes.or(login_route);
    #[cfg(feature = "frontend")]
    let routes = routes.or(page_routes);
    let routes = routes.with(warp::trace::trace(|info| {
        let remote_addr = info
            .remote_addr()
            .unwrap_or_else(|| std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, 0)))
            .ip();
        let span = tracing::info_span!("request", %remote_addr);
        span.in_scope(|| {
            tracing::info!("Request to {}", info.path());
            tracing::debug!(
                "by {}, using {} {:?}",
                info.user_agent().unwrap_or("unknown"),
                remote_addr,
                info.version(),
            );
        });
        span
    }));

    if CONFIG.tls {
        warp::serve(routes)
            .tls()
            .cert_path(&CONFIG.cert)
            .key_path(&CONFIG.key)
            .run((addr, CONFIG.port))
            .await;
    } else {
        warp::serve(routes).run((addr, CONFIG.port)).await;
    }*/

    let addr = std::net::SocketAddr::from((IpAddr::from([0; 8]), 5252));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, std::convert::Infallible>(service_fn(|req| async {
            let mut response = Response::new(Body::empty());

            match (req.method(), req.uri().path().trim_end_matches('/')) {
                (&Method::GET, "/favicon.png") => response = routes::favicon_route(req)?,
                (&Method::GET, path) if path.starts_with("/assets") => {
                    response = routes::assets_route(req)?;
                }
                (&Method::GET, "/ws") => {
                    response = routes::websocket(req, socket_handlers::socket_handler)?;
                }
                (&Method::POST, "/login") => {
                    response = routes::login_route(req).await?;
                }
                (&Method::GET, _) => {
                    response = routes::main_route(req)?;
                }
                _ => *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED,
            }

            anyhow::Ok::<_>(response)
        }))
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
