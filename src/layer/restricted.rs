use crate::lib::jwt::Auth;
use axum::http::{Request, Response};
use hyper::{body::HttpBody, http::StatusCode, Body as HyperBody, Error as HyperError, HeaderMap};
use bytes::Bytes;
use futures_util::ready;
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

const AUTH_HEADER: &'static str = "Authorization";

pub struct Restricted<T> {
    inner: T,
}

impl<T> Restricted<T> {
    fn new(inner: T) -> Restricted<T> {
        Self { inner }
    }
    fn authorize<B>(&self, req: &Request<B>) -> Option<Auth> {
        if let Some(auth_header) = req.headers().get(AUTH_HEADER) {
            Some(Auth {
                id: "1".to_string(),
                username: "aliz".to_string(),
            })
        } else {
            None
        }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for Restricted<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    ResBody: HttpBody + Default,
{
    type Response = Response<ResBody>;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future, ResBody>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        if let Some(auth) = self.authorize(&req) {
            req.extensions_mut().insert(auth);
            ResponseFuture::next(self.inner.call(req))
        } else {
            let body = ResBody::default();
            let body = body.map_data(|v|{
              // &b"hello world"[..]
              "123".as_bytes()
            }).into_inner();
            let resp = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(body)
                .unwrap();
            ResponseFuture::failed(resp)
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F, B> {
    #[pin]
    kind: Kind<F, B>,
}

impl<F, B> ResponseFuture<F, B> {
    fn next(fut: F) -> Self {
        Self {
            kind: Kind::Future(fut),
        }
    }
    fn failed(resp: Response<B>) -> Self {
        Self {
            kind: Kind::Error(Some(resp)),
        }
    }
}

#[pin_project(project= KindProj)]
enum Kind<F, B> {
    Future(#[pin] F),
    Error(Option<Response<B>>),
}

impl<F, B, E> Future for ResponseFuture<F, B>
where
    F: Future<Output = Result<Response<B>, E>>,
{
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().kind.project() {
            KindProj::Future(future) => future.poll(cx),
            KindProj::Error(resp) => {
                let resp = resp.take().unwrap();
                // let resp = Response::new("hi");
                Poll::Ready(Ok(resp))
            }
        }
    }
}

pub struct RestrictedLayer;

impl RestrictedLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for RestrictedLayer {
    type Service = Restricted<S>;
    fn layer(&self, inner: S) -> Self::Service {
        Restricted::new(inner)
    }
}

pub struct RestrictBody{
  inner: HyperBody
}

impl RestrictBody{
  fn new(inner: HyperBody) -> Self{
    Self{ inner }
  }
}

impl HttpBody for RestrictBody{
  type Data = Bytes;
  type Error = HyperError;
  fn poll_data(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Self::Data, Self::Error>>> {
      if let Some(chunk) = ready!(Pin::new(&mut self.inner).poll_data(cx)?) {
          Poll::Ready(Some(Ok(chunk)))
      } else {
        Poll::Ready(None)
      }
  }
  fn poll_trailers(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
      Pin::new(&mut self.inner).poll_trailers(cx)
  }
}