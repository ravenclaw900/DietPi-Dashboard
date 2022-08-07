#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
use crate::shared::CONFIG;
use anyhow::Context;
use futures::FutureExt;
use hyper::service::{make_service_fn, service_fn};
use std::task::Poll;
use std::{net::IpAddr, str::FromStr};

mod config;
mod page_handlers;
mod routes;
mod shared;
mod socket_handlers;
mod systemdata;

#[cfg(feature = "frontend")]
const DIR: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");

struct HyperTLSAcceptor {
    listener: tokio::net::TcpListener,
    acceptor: tokio_rustls::TlsAcceptor,
    accept_future: Option<tokio_rustls::Accept<tokio::net::TcpStream>>,
}

impl hyper::server::accept::Accept for HyperTLSAcceptor {
    type Conn = tokio_rustls::server::TlsStream<tokio::net::TcpStream>;
    type Error = std::io::Error;

    fn poll_accept(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        if self.accept_future.is_none() {
            match self.listener.poll_accept(cx) {
                Poll::Ready(stream) => match stream {
                    Ok(stream) => self.accept_future = Some(self.acceptor.accept(stream.0)),
                    Err(err) => return Poll::Ready(Some(Err(err))),
                },
                Poll::Pending => return Poll::Pending,
            }
        }
        if let Some(accept_future) = &mut self.accept_future {
            match accept_future.poll_unpin(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(tls) => {
                    self.accept_future = None;
                    let tls = match tls {
                        Ok(tls) => tls,
                        Err(err) => return Poll::Ready(Some(Err(err))),
                    };
                    return Poll::Ready(Some(Ok(tls)));
                }
            }
        }
        Poll::Pending
    }
}

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

        let tls_listener = HyperTLSAcceptor {
            listener: tokio::net::TcpListener::bind(&addr)
                .await
                .with_context(|| format!("Couldn't bind to {}", &addr))?,
            acceptor: tokio_rustls::TlsAcceptor::from(tls_cfg),
            accept_future: None,
        };

        hyper::server::Server::builder(tls_listener)
            .serve(make_service_fn(|_conn| async {
                Ok::<_, std::convert::Infallible>(service_fn(|req| async {
                    routes::router(req).await
                }))
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
