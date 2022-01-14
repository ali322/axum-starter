use crate::util::jwt::{decode_token};
use axum::{
    body::{self, BoxBody},
    http::{Request, StatusCode},
    response::Response,
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

impl<B> AuthorizeRequest<B> for Restrict {
    type ResponseBody = BoxBody;
    fn authorize(&mut self, req: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        if let Some(auth_string) = req.headers().get(AUTH_HEADER) {
            let auth_str = auth_string.to_str().unwrap();
            if auth_str.starts_with("Bearer ") {
                let auth_str = auth_str.replace("Bearer ", "");
                let decoded = decode_token(&auth_str);
                match decoded {
                    Ok(token_data) => {
                      let auth = token_data.claims.auth;
                      req.extensions_mut().insert(auth);
                    },
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
        if let Some(reject_reason) = self.reject_reason.clone() {
          let json_body = json!({"code":-1, "message": reject_reason}).to_string();
                let body = body::boxed(body::Full::from(json_body));
                let unauthorized_response = Response::builder()
                    .status(StatusCode::OK)
                    .body(body)
                    .unwrap();
                    Err(unauthorized_response)
        } else {
          Ok(())
        }
    }
}
