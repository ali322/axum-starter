use axum::http::{Request, Response, header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_ALLOW_METHODS}};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Debug, Clone)]
pub struct Cors<S> {
    pub inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for Cors<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<S::Response, S::Error>>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let mut res: Response<ResBody> = inner.call(req).await?;
            res.headers_mut()
                .insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
            res.headers_mut().insert(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization, Content-Length, X-Requested-With, Accept, x-csrf-token, origin".parse().unwrap());
            res.headers_mut().insert(ACCESS_CONTROL_ALLOW_METHODS, "GET,PUT,POST,DELETE,OPTIONS".parse().unwrap());
            Ok(res)
        })
    }
}
