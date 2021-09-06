use axum::{
  body::{box_body, Body, BoxBody},
  http::{Request, Response, StatusCode},
};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Debug, Clone)]
pub struct Todo<S> {
  inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for Todo<S>
where
  S: Service<Request<ReqBody>, Response = Response<BoxBody>> + Clone + Send + 'static,
  S::Future: Send + 'static,
  ReqBody: Send + 'static,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = BoxFuture<'static, Result<S::Response, S::Error>>;
  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
      self.inner.poll_ready(cx)
  }
  fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
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
