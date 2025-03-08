use std::future::Future;
use std::pin::Pin;
use axum::{
    response::Response,
    body::Body,
    extract::Request,
};
use tower::{Service, Layer};
use std::task::{Context, Poll};
use tokio::time::Instant;
use tracing::warn;

#[derive(Clone)]
pub struct ServerTimeLayer;


const SERVER_TIME_HEADER : &str = "x-server-time";
impl<S> Layer<S> for ServerTimeLayer {
    type Service = ServerTimeMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ServerTimeMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct ServerTimeMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for ServerTimeMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = Pin<Box<dyn Future<Output =Result<Self::Response,Self::Error>>  + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let start = Instant::now();
        let future = self.inner.call(request);
        Box::pin(async move {
            let mut res: Response = future.await?;
            let elapsed = format!("{}us",start.elapsed().as_micros());
            match elapsed.parse() {
                Ok(v) => {
                    res.headers_mut().insert(SERVER_TIME_HEADER,v);
                }
                Err(e) => {
                    warn!("Parse elapsed time error: {}", e);
                }
            }
            Ok(res)
        })
    }
}