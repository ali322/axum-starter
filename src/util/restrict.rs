use crate::util::jwt::{decode_token, Auth};
use axum::{
    body::{box_body, Body, BoxBody},
    http::{Request, Response, StatusCode},
};
use serde_json::json;
use tower_http::auth::AuthorizeRequest;

const AUTH_HEADER: &'static str = "Authorization";

#[derive(Debug, Clone)]
pub struct Restrict {
    reject_reason: Option<String>,
}

impl Restrict {
    pub fn new() -> Self {
        Self {
            reject_reason: None,
        }
    }
}

impl AuthorizeRequest for Restrict {
    type Output = Auth;
    type ResponseBody = BoxBody;
    fn authorize<B>(&mut self, req: &Request<B>) -> Option<Self::Output> {
        let mut output: Option<Self::Output> = None;
        if let Some(auth_string) = req.headers().get(AUTH_HEADER) {
            let auth_str = auth_string.to_str().unwrap();
            if auth_str.starts_with("Bearer ") {
                let auth_str = auth_str.replace("Bearer ", "");
                let decoded = decode_token(&auth_str);
                match decoded {
                    Ok(token_data) => output = Some(token_data.claims.auth),
                    Err(e) => {
                        self.reject_reason = Some(format!("请求头 Authorization 解析错误: {:?}", e))
                    }
                }
            } else {
                self.reject_reason = Some("请求头 Authorization 不合法".to_string());
            }
        } else {
            self.reject_reason = Some("请求头 Authorization 不能为空".to_string());
        }
        output
    }
    fn on_authorized<B>(&mut self, req: &mut Request<B>, output: Self::Output) {
        req.extensions_mut().insert(output);
    }
    fn unauthorized_response<B>(&mut self, _req: &Request<B>) -> Response<Self::ResponseBody> {
        let json_body = json!({"code":-1, "message": self.reject_reason}).to_string();
        let body = box_body(Body::from(json_body));
        let mut res = Response::new(body);
        *res.status_mut() = StatusCode::OK;
        res
    }
}