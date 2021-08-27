use axum::{
    body::{box_body, BoxBody, HttpBody},
    http::{Request, Response, StatusCode},
};
use futures::future::BoxFuture;
use hyper::Body;
use std::{future::Future, mem, pin::Pin, str::FromStr, task::{Context, Poll}};
use tower::Service;
use tower_http::auth::AuthorizeRequest;

use crate::util::restrict::Restrict;

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

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        println!("`TodoMiddleware` called!");

        // best practice is to clone the inner service like this
        // see https://github.com/tower-rs/tower/issues/547 for details
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            // let mut res: Response<ResBody> = inner.call(req).await?;
            let body = box_body(Body::from("other_body"));
            let mut res = Response::new(body);
            *res.status_mut() = StatusCode::OK;
            // println!("`TodoMiddleware` received the response");
            Ok(res)
        })
    }
}

