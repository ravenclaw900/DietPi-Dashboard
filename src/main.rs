#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::too_many_lines)]
#![warn(rust_2018_idioms)]
use crate::shared::CONFIG;
use anyhow::Context;
use hyper::service::{make_service_fn, service_fn};
use smol::future::FutureExt;
use smol::io::{AsyncRead, AsyncWrite};
use smol::stream::StreamExt;
use std::net::Shutdown;
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

#[derive(Clone)]
struct SmolExecutor;

impl<F: smol::future::Future + Send + 'static> hyper::rt::Executor<F> for SmolExecutor {
    fn execute(&self, fut: F) {
        smol::spawn(async { drop(fut.await) }).detach();
    }
}

struct SmolHyperAcceptor<'a> {
    incoming: smol::net::Incoming<'a>,
    acceptor: Option<async_rustls::TlsAcceptor>,
    accept_future: Option<async_rustls::Accept<smol::net::TcpStream>>,
}

impl<'a> SmolHyperAcceptor<'a> {
    const fn new(
        incoming: smol::net::Incoming<'a>,
        acceptor: Option<async_rustls::TlsAcceptor>,
    ) -> Self {
        Self {
            incoming,
            acceptor,
            accept_future: None,
        }
    }
}

impl hyper::server::accept::Accept for &mut SmolHyperAcceptor<'_> {
    type Conn = SmolTcpAdaptor;
    type Error = anyhow::Error;

    fn poll_accept(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        if self.accept_future.is_none() {
            match smol::ready!(self.incoming.poll_next(cx)) {
                Some(Ok(stream)) => {
                    if let Some(acceptor) = &self.acceptor {
                        self.accept_future = Some(acceptor.accept(stream));
                    } else {
                        return Poll::Ready(Some(Ok(SmolTcpAdaptor::Plain(stream))));
                    }
                }
                Some(Err(err)) => {
                    return Poll::Ready(Some(Err(err).context("Couldn't make TCP connection")));
                }
                None => return Poll::Ready(None),
            }
        }
        if let Some(accept_future) = &mut self.accept_future {
            let tls = smol::ready!(accept_future.poll(cx));
            self.accept_future = None;
            let tls = match tls {
                Ok(tls) => tls,
                Err(err) => {
                    return Poll::Ready(Some(Err(err).context("Couldn't encrypt TCP connection")));
                }
            };
            return Poll::Ready(Some(Ok(SmolTcpAdaptor::Tls(Box::new(tls)))));
        }
        Poll::Pending
    }
}

enum SmolTcpAdaptor {
    Plain(smol::net::TcpStream),
    Tls(Box<async_rustls::server::TlsStream<smol::net::TcpStream>>),
}

impl SmolTcpAdaptor {
    fn remote_addr(&self) -> std::net::SocketAddr {
        match self {
            Self::Plain(tcp) => tcp.peer_addr(),
            Self::Tls(tls) => tls.get_ref().0.peer_addr(),
        }
        .unwrap_or_else(|_| std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, 0)))
    }
}

impl tokio::io::AsyncRead for SmolTcpAdaptor {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            Self::Plain(s) => {
                return std::pin::Pin::new(s)
                    .poll_read(cx, buf.initialize_unfilled())
                    .map_ok(|size| {
                        buf.advance(size);
                    });
            }
            Self::Tls(s) => {
                return std::pin::Pin::new(s)
                    .poll_read(cx, buf.initialize_unfilled())
                    .map_ok(|size| {
                        buf.advance(size);
                    });
            }
        }
    }
}

impl tokio::io::AsyncWrite for SmolTcpAdaptor {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match &mut *self {
            Self::Plain(s) => std::pin::Pin::new(s).poll_write(cx, buf),
            Self::Tls(s) => std::pin::Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            Self::Plain(s) => std::pin::Pin::new(s).poll_flush(cx),
            Self::Tls(s) => std::pin::Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            Self::Plain(s) => {
                s.shutdown(Shutdown::Write)?;
                Poll::Ready(Ok(()))
            }
            Self::Tls(s) => std::pin::Pin::new(s).poll_close(cx),
        }
    }
}

fn main() -> anyhow::Result<()> {
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

    let make_svc = make_service_fn(|conn: &SmolTcpAdaptor| {
        let remote_addr = conn.remote_addr();
        async move {
            Ok::<_, std::convert::Infallible>(service_fn(move |req| {
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
                routes::router(req, span)
            }))
        }
    });

    smol::block_on(async {
        let tcp = smol::net::TcpListener::bind(&addr)
            .await
            .with_context(|| format!("Couldn't bind to {}", &addr))?;

        if CONFIG.tls {
            let tls_cfg = {
                let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(
                    std::fs::File::open(&CONFIG.cert).context("Couldn't open cert file")?,
                ))
                .context("Couldn't read certs")?
                .into_iter()
                .map(async_rustls::rustls::Certificate)
                .collect();

                let key = match rustls_pemfile::read_one(&mut std::io::BufReader::new(
                    std::fs::File::open(&CONFIG.key).context("Couldn't open cert file")?,
                ))
                .context("Couldn't read key")?
                .context("No private key")?
                {
                    rustls_pemfile::Item::PKCS8Key(vec) | rustls_pemfile::Item::RSAKey(vec) => {
                        async_rustls::rustls::PrivateKey(vec)
                    }
                    _ => anyhow::bail!("No PKCS8 or RSA formatted private key"),
                };

                let mut cfg = async_rustls::rustls::ServerConfig::new(
                    async_rustls::rustls::NoClientAuth::new(),
                );
                cfg.set_single_cert(certs, key)
                    .context("Couldn't build TLS config")?;
                cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
                std::sync::Arc::new(cfg)
            };

            let mut tls_listener = SmolHyperAcceptor::new(
                tcp.incoming(),
                Some(async_rustls::TlsAcceptor::from(tls_cfg)),
            );

            // Ignore result, because it will never be an error
            loop {
                let _res = hyper::server::Server::builder(&mut tls_listener)
                    .executor(SmolExecutor)
                    .serve(make_svc)
                    .await
                    .context("HTTPS server error")
                    .or_else(|e| {
                        tracing::warn!("{:?}", e);
                        anyhow::Ok(())
                    });
            }
        } else {
            hyper::server::Server::builder(&mut SmolHyperAcceptor::new(tcp.incoming(), None))
                .executor(SmolExecutor)
                .serve(make_svc)
                .await
                .context("HTTP server error")?;
        }
        Ok(())
    })
}
