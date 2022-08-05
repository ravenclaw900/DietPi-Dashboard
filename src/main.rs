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
                .finish(),
        )
        .context("Couldn't init logger")?;
    }

    let addr = std::net::SocketAddr::from((IpAddr::from([0; 8]), CONFIG.port));

    if CONFIG.tls {
        simple_hyper_server_tls::hyper_from_pem_files(
            &CONFIG.cert,
            &CONFIG.key,
            simple_hyper_server_tls::Protocols::HTTP1,
            &addr,
        )
        .map_err(|e| anyhow::anyhow!(e.to_string()))
        .context("Couldn't set up TLS server")?
        .serve(make_service_fn(|_conn| async {
            Ok::<_, std::convert::Infallible>(service_fn(|req| async { routes::router(req).await }))
        }))
        .await
        .context("HTTPS server error")?;
    } else {
        hyper::server::Server::try_bind(&addr)
            .with_context(|| format!("Couldn't bind to {}", addr))?
            .serve(make_service_fn(|_conn| async {
                Ok::<_, std::convert::Infallible>(service_fn(|req| async {
                    routes::router(req).await
                }))
            }))
            .await
            .context("HTTP server error")?;
    }

    Ok(())
}
