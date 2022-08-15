#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::too_many_lines)]
#![warn(rust_2018_idioms)]
use crate::shared::CONFIG;
use anyhow::Context;
use futures::FutureExt;
use hyper::server::conn::AddrIncoming;
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
static DIR: include_dir::Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");

struct ConnWithAddr {
    conn: tokio_rustls::server::TlsStream<tokio::net::TcpStream>,
    addr: std::net::SocketAddr,
}

struct HyperTlsAcceptor {
    listener: tokio::net::TcpListener,
    acceptor: tokio_rustls::TlsAcceptor,
    accept_future: Option<tokio_rustls::Accept<tokio::net::TcpStream>>,
    remote_addr: Option<std::net::SocketAddr>,
}

impl tokio::io::AsyncRead for ConnWithAddr {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.conn).poll_read(cx, buf)
    }
}

impl tokio::io::AsyncWrite for ConnWithAddr {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        std::pin::Pin::new(&mut self.conn).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.conn).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.conn).poll_shutdown(cx)
    }
}

impl hyper::server::accept::Accept for &mut HyperTlsAcceptor {
    type Conn = ConnWithAddr;
    type Error = anyhow::Error;

    fn poll_accept(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        if self.accept_future.is_none() {
            match self.listener.poll_accept(cx) {
                Poll::Ready(stream) => match stream {
                    Ok(stream) => {
                        self.accept_future = Some(self.acceptor.accept(stream.0));
                        self.remote_addr = Some(stream.1);
                    }
                    Err(err) => {
                        return Poll::Ready(Some(Err(err).context("Couldn't make TCP connection")));
                    }
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
                        Err(err) => {
                            return Poll::Ready(Some(
                                Err(err).context("Couldn't encrypt TCP connection"),
                            ));
                        }
                    };
                    let remote_addr = self.remote_addr.take().unwrap_or_else(|| {
                        std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, 0))
                    });
                    return Poll::Ready(Some(Ok(ConnWithAddr {
                        conn: tls,
                        addr: remote_addr,
                    })));
                }
            }
        }
        Poll::Pending
    }
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

    let tcp = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Couldn't bind to {}", &addr))?;

    let make_svc = |remote_addr| async move {
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
    };

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

        let mut tls_listener = HyperTlsAcceptor {
            listener: tcp,
            acceptor: tokio_rustls::TlsAcceptor::from(tls_cfg),
            accept_future: None,
            remote_addr: None,
        };

        // Ignore result, because it will never be an error
        loop {
            let _res = hyper::server::Server::builder(&mut tls_listener)
                .serve(make_service_fn(|conn: &ConnWithAddr| {
                    let remote_addr = conn.addr;
                    make_svc(remote_addr)
                }))
                .await
                .context("HTTPS server error")
                .or_else(|e| {
                    tracing::warn!("{:?}", e);
                    anyhow::Ok(())
                });
        }
    } else {
        hyper::server::Server::builder(
            AddrIncoming::from_listener(tcp).context("Couldn't convert TCP listener")?,
        )
        .serve(make_service_fn(|conn: &hyper::server::conn::AddrStream| {
            let remote_addr = conn.remote_addr();
            make_svc(remote_addr)
        }))
        .await
        .context("HTTP server error")?;
    }

    Ok(())
}
