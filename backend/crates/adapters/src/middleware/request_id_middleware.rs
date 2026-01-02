use axum::{
    body::Body,
    http::{Request, Response},
    response::IntoResponse,
};
use futures_util::future::BoxFuture;
use std::{
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};
use uuid::Uuid;

use crate::http::request_id::RequestId;

#[derive(Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RequestIdMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let id = Uuid::new_v4();
        req.extensions_mut().insert(RequestId(id));

        let future = self.inner.call(req);
        Box::pin(async move {
            let start = Instant::now();
            let mut res = future.await?;
            let latency = start.elapsed();
            res.headers_mut()
                .insert("x-request-id", id.to_string().parse().unwrap());
            tracing::debug!("Request took {}ms", latency.as_millis());
            Ok(res.into_response())
        })
    }
}
