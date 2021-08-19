use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use crate::lib::jwt::Auth;
use axum::http::HeaderValue;
use bytes::Bytes;
use futures_util::ready;
use http_body::combinators::BoxBody;
use hyper::{
    body::HttpBody, http::StatusCode, Body as HyperBody, Error as HyperError, HeaderMap, Request,
    Response
};
use pin_project::pin_project;
use tower_http::{auth::AuthorizeRequest};

#[allow(unused_macros)]
macro_rules! opaque_body {
    ($(#[$m:meta])* pub type $name:ident = $actual:ty;) => {
        opaque_body! {
            $(#[$m])* pub type $name<> = $actual;
        }
    };

    ($(#[$m:meta])* pub type $name:ident<$($param:ident),*> = $actual:ty;) => {
        #[pin_project::pin_project]
        $(#[$m])*
        pub struct $name<$($param),*>(#[pin] pub(crate) $actual);

        impl<$($param),*> http_body::Body for $name<$($param),*> {
            type Data = <$actual as http_body::Body>::Data;
            type Error = <$actual as http_body::Body>::Error;

            #[inline]
            fn poll_data(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Result<Self::Data, Self::Error>>> {
                self.project().0.poll_data(cx)
            }

            #[inline]
            fn poll_trailers(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Result<Option<HeaderMap>, Self::Error>> {
                self.project().0.poll_trailers(cx)
            }

            #[inline]
            fn is_end_stream(&self) -> bool {
                http_body::Body::is_end_stream(&self.0)
            }

            #[inline]
            fn size_hint(&self) -> http_body::SizeHint {
                http_body::Body::size_hint(&self.0)
            }
        }
    };
}

const AUTH_HEADER: &'static str = "Authorization";

pub struct Restrict<ResBody> {
    _ty: PhantomData<fn() -> ResBody>,
}

impl<ResBody> Restrict<ResBody> {
    pub fn new() -> Self
    where
        ResBody: HttpBody + Default,
    {
        Self { _ty: PhantomData }
    }
}

impl<ResBody> Clone for Restrict<ResBody> {
    fn clone(&self) -> Self {
        Self { _ty: PhantomData }
    }
}

impl<ResBody> AuthorizeRequest for Restrict<ResBody>
where
    ResBody: HttpBody + Default,
{
    type Output = Auth;
    type ResponseBody = ResBody;
    fn authorize<B>(&mut self, req: &Request<B>) -> Option<Self::Output> {
        if let Some(auth_header) = req.headers().get(AUTH_HEADER) {
            Some(Auth {
                id: "1".to_string(),
                username: "aliz".to_string(),
            })
        } else {
            None
        }
    }
    fn on_authorized<B>(&mut self, req: &mut Request<B>, output: Self::Output) {
        req.extensions_mut().insert(output);
    }
    fn unauthorized_response<B>(&mut self, req: &Request<B>) -> Response<Self::ResponseBody> {
        // let body = Body::from("unauthorized");
        let body = ResBody::default();
        // let body = ResponseBody(HyperBody::from("123").boxed());
        // let body = RestrictBody::new("123".into());
        // let body = body.map_data(|v|{
        //   &b"hello world"[..]
        //   // "123".as_bytes()
        // }).into_inner();
        let mut res = Response::new(body);
        *res.status_mut() = StatusCode::UNAUTHORIZED;
        // res.headers_mut().insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        // *res.body_mut().push_str("hi");
        res
    }
}

opaque_body!{
  pub type ResponseBody = BoxBody<Bytes, HyperError>;
}
