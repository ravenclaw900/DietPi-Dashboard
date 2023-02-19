#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::too_many_lines)]
#![warn(rust_2018_idioms)]
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
mod types;

#[cfg(feature = "frontend")]
static DIR: include_dir::Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");

enum TlsOrTcpConnection {
    Plain(tokio::net::TcpStream, std::net::SocketAddr),
    Tls(
        Box<tokio_rustls::server::TlsStream<tokio::net::TcpStream>>,
        std::net::SocketAddr,
    ),
}

struct HyperTlsOrTcpAcceptor {
    listener: tokio::net::TcpListener,
    acceptor: Option<tokio_rustls::TlsAcceptor>,
    accept_future: Option<tokio_rustls::Accept<tokio::net::TcpStream>>,
    remote_addr: Option<std::net::SocketAddr>,
}

impl HyperTlsOrTcpAcceptor {
    const fn new(
        listener: tokio::net::TcpListener,
        acceptor: Option<tokio_rustls::TlsAcceptor>,
    ) -> Self {
        Self {
            listener,
            acceptor,
            accept_future: None,
            remote_addr: None,
        }
    }
}

impl TlsOrTcpConnection {
    const fn remote_addr(&self) -> std::net::SocketAddr {
        match self {
            Self::Plain(_, remote_addr) | Self::Tls(_, remote_addr) => *remote_addr,
        }
    }
}

impl tokio::io::AsyncRead for TlsOrTcpConnection {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Plain(tcp, _) => std::pin::Pin::new(tcp).poll_read(cx, buf),
            Self::Tls(tls, _) => std::pin::Pin::new(tls).poll_read(cx, buf),
        }
    }
}

impl tokio::io::AsyncWrite for TlsOrTcpConnection {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match self.get_mut() {
            Self::Plain(tcp, _) => std::pin::Pin::new(tcp).poll_write(cx, buf),
            Self::Tls(tls, _) => std::pin::Pin::new(tls).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::Plain(tcp, _) => std::pin::Pin::new(tcp).poll_flush(cx),
            Self::Tls(tls, _) => std::pin::Pin::new(tls).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::Plain(tcp, _) => std::pin::Pin::new(tcp).poll_shutdown(cx),
            Self::Tls(tls, _) => std::pin::Pin::new(tls).poll_shutdown(cx),
        }
    }
}

impl hyper::server::accept::Accept for &mut HyperTlsOrTcpAcceptor {
    type Conn = TlsOrTcpConnection;
    type Error = anyhow::Error;

    fn poll_accept(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        if self.accept_future.is_none() {
            match self.listener.poll_accept(cx) {
                Poll::Ready(stream) => match stream {
                    Ok(stream) => {
                        if let Some(acceptor) = &self.acceptor {
                            self.accept_future = Some(acceptor.accept(stream.0));
                            self.remote_addr = Some(stream.1);
                        } else {
                            return Poll::Ready(Some(Ok(TlsOrTcpConnection::Plain(
                                stream.0, stream.1,
                            ))));
                        }
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
                    return Poll::Ready(Some(Ok(TlsOrTcpConnection::Tls(
                        Box::new(tls),
                        remote_addr,
                    ))));
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

    let make_svc = make_service_fn(|conn: &TlsOrTcpConnection| {
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

        HyperTlsOrTcpAcceptor::new(tcp, Some(tokio_rustls::TlsAcceptor::from(tls_cfg)))
    } else {
        HyperTlsOrTcpAcceptor::new(tcp, None)
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
