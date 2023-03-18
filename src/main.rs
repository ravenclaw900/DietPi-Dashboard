#![warn(
    clippy::pedantic,
    clippy::unwrap_used,
    rust_2018_idioms,
    clippy::nursery
)]
#![allow(clippy::too_many_lines)]
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
mod tls;

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

    let make_svc = make_service_fn(|conn: &tls::TlsOrTcpConnection| {
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

    let mut acceptor = if CONFIG.tls {
        let tls_cfg = {
            let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(
                std::fs::File::open(&CONFIG.cert).context("Couldn't open cert file")?,
            ))
            .context("Couldn't read certs")?
            .into_iter()
            .map(tokio_rustls::rustls::Certificate)
            .collect();

            let key = match rustls_pemfile::read_one(&mut std::io::BufReader::new(
                std::fs::File::open(&CONFIG.key).context("Couldn't open cert file")?,
            ))
            .context("Couldn't read key")?
            .context("No private key")?
            {
                rustls_pemfile::Item::PKCS8Key(vec) | rustls_pemfile::Item::RSAKey(vec) => {
                    tokio_rustls::rustls::PrivateKey(vec)
                }
                _ => anyhow::bail!("No PKCS8 or RSA formatted private key"),
            };

            let mut cfg = tokio_rustls::rustls::ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(certs, key)
                .context("Couldn't build TLS config")?;
            cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
            std::sync::Arc::new(cfg)
        };

        tls::HyperTlsOrTcpAcceptor::new(tcp, Some(tokio_rustls::TlsAcceptor::from(tls_cfg)))
    } else {
        tls::HyperTlsOrTcpAcceptor::new(tcp, None)
    };

    // Ignore result, because it will never be an error
    loop {
        let _res = hyper::server::Server::builder(&mut acceptor)
            .serve(make_svc)
            .await
            .context("Server error")
            .or_else(|e| {
                tracing::warn!("{:?}", e);
                anyhow::Ok(())
            });
    }
}
