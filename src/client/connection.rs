use futures::{Future, Poll};
use http::{Request, Response};
use hyper::body::Payload;
use hyper::client::conn;
use tokio_io::{AsyncRead, AsyncWrite};
use tower_direct_service::DirectService;

pub struct Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: Payload,
{
    sender: conn::SendRequest<B>,
    conn: conn::Connection<T, B>,
}

impl<T, B> Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: Payload,
{
    pub(crate) fn new(sender: conn::SendRequest<B>, conn: conn::Connection<T, B>) -> Self {
        Connection { sender, conn }
    }
}

impl<T, B> DirectService<Request<B>> for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send,
    B: Payload,
{
    type Response = Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = conn::ResponseFuture;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.sender.poll_ready()
    }

    fn poll_service(&mut self) -> Poll<(), Self::Error> {
        self.conn.poll()
    }

    fn poll_close(&mut self) -> Poll<(), Self::Error> {
        unimplemented!()
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        self.sender.send_request(req)
    }
}
