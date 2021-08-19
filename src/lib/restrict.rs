use std::{
    marker::PhantomData,
};

use crate::lib::jwt::Auth;
use hyper::{
    body::HttpBody, http::StatusCode, Request,
    Response
};
use tower_http::{auth::AuthorizeRequest};

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
        let body = ResBody::default();
        let mut res = Response::new(body);
        *res.status_mut() = StatusCode::UNAUTHORIZED;
        res
    }
}

