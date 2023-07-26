#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::too_many_lines)]
#![warn(rust_2018_idioms)]
use crate::shared::CONFIG;
use anyhow::Context;
use hyper::service::{make_service_fn, service_fn};
use std::{net::IpAddr, str::FromStr};

mod config;
mod page_handlers;
mod routes;
mod shared;
mod socket_handlers;
mod systemdata;

#[cfg(feature = "frontend")]
static DIR: include_dir::Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");

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

    let tcp = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Couldn't bind to {}", &addr))?;

    let make_svc = make_service_fn(|conn: &flexible_hyper_server_tls::HttpOrHttpsConnection| {
        let remote_addr = conn.remote_addr();
        async move {
            Ok::<_, std::convert::Infallible>(service_fn(move |req| async move {
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
            }))
        }
    });

    let acceptor = if CONFIG.tls {
        let tls_acceptor = flexible_hyper_server_tls::tlsconfig::get_tlsacceptor_from_files(
            &CONFIG.cert,
            &CONFIG.key,
            flexible_hyper_server_tls::tlsconfig::HttpProtocol::Http1,
        )
        .context("Couldn't get TLS config")?;

        flexible_hyper_server_tls::HyperHttpOrHttpsAcceptor::new_https(
            tcp,
            tls_acceptor,
            std::time::Duration::from_secs(10),
        )
    } else {
        flexible_hyper_server_tls::HyperHttpOrHttpsAcceptor::new_http(tcp)
    };

    let mut server = hyper::server::Server::builder(acceptor).serve(make_svc);

    // Ignore result, because it will never be an error
    loop {
        let _res = (&mut server).await.context("Server error").or_else(|e| {
            tracing::warn!("{:?}", e);
            anyhow::Ok(())
        });
    }
}
