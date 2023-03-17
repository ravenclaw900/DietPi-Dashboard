use anyhow::Context;
use futures::FutureExt;
use std::task::Poll;

#[allow(clippy::module_name_repetitions)]
pub enum TlsOrTcpConnection {
    Plain(tokio::net::TcpStream, std::net::SocketAddr),
    Tls(
        Box<tokio_rustls::server::TlsStream<tokio::net::TcpStream>>,
        std::net::SocketAddr,
    ),
}

pub struct HyperTlsOrTcpAcceptor {
    listener: tokio::net::TcpListener,
    acceptor: Option<tokio_rustls::TlsAcceptor>,
    accept_future: Option<tokio_rustls::Accept<tokio::net::TcpStream>>,
    remote_addr: Option<std::net::SocketAddr>,
}

impl HyperTlsOrTcpAcceptor {
    pub const fn new(
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
    pub const fn remote_addr(&self) -> std::net::SocketAddr {
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
