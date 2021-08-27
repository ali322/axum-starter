use axum::{
    body::{box_body, Body, BoxBody},
    http::{Request, Response, StatusCode},
};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
pub struct TodoMiddleware<S> {
    pub inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for TodoMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut _req: Request<ReqBody>) -> Self::Future {
        // let clone = self.inner.clone();
        // let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            // let mut res: Response<ResBody> = inner.call(req).await?;
            let body = box_body(Body::from("other_body"));
            // let mut res = Response::new(body);
            // *res.status_mut() = StatusCode::OK;
            let res = Response::builder()
                .status(StatusCode::OK)
                .body(body)
                .unwrap();
            // println!("`TodoMiddleware` received the response");
            Ok(res)
        })
    }
}
