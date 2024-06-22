#![warn(
    clippy::pedantic,
    clippy::unwrap_used,
    rust_2018_idioms,
    clippy::nursery
)]
#![allow(clippy::too_many_lines)]
use crate::shared::CONFIG;
use anyhow::Context;
use hyper::{body::Incoming, service::service_fn, Error, Request, Response};
use std::{net::IpAddr, str::FromStr};

mod config;
mod page_handlers;
mod routes;
mod shared;
mod socket_handlers;
mod systemdata;

async fn svc(
    req: Request<Incoming>,
) -> anyhow::Result<Response<http_body_util::combinators::BoxBody<bytes::Bytes, Error>>> {
    // ToDo: How to get client addr, or can we skip it?
    let remote_addr = "127.0.0.1";
    let span = tracing::info_span!("request", %remote_addr);
    span.in_scope(|| {
        tracing::info!("Request to {}", req.uri().path());
        tracing::debug!(
            "using {:?}",
            req.headers()
                .get(hyper::header::USER_AGENT)
                .map_or("unknown", |x| x.to_str().unwrap_or("unknown"))
        );
    });
    routes::router(req, span).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(
                tracing_subscriber::filter::LevelFilter::from_str(&CONFIG.log_level)
                    .context("Couldn't parse log level")?,
            )
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .finish(),
    )
    .context("Couldn't init logger")?;

    let addr = std::net::SocketAddr::from((IpAddr::from([0; 8]), CONFIG.port));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Couldn't bind to {}", &addr))?;

    let builder = flexible_hyper_server_tls::AcceptorBuilder::new(listener);

    let mut acceptor = if CONFIG.tls {
        let tls_acceptor = flexible_hyper_server_tls::rustls_helpers::get_tlsacceptor_from_files(
            &CONFIG.cert,
            &CONFIG.key,
        )
        .context("Couldn't get TLS config")?;

        builder.https(tls_acceptor).build()
    } else {
        builder.build()
    };

    // Ignore result, because it will never be an error
    loop {
        acceptor.accept(service_fn(svc)).await;
    }
}
