use super::APIState;
use crate::{
    repository::user::*,
    util::{jwt::generate_token, APIResult},
};
use axum::extract::{Extension, Json};
use std::sync::Arc;
use validator::Validate;

pub async fn register(
    Json(body): Json<NewUser>,
    Extension(state): Extension<Arc<APIState>>,
) -> APIResult {
    body.validate()?;
    if body.exists(&state.conn).await.is_ok() {
        return Err(reject!("用户已存在"));
    }
    let user = body.create(&state.conn).await?.unwrap();
    let token = generate_token(user["id"].to_string(), user["username"].to_string());
    Ok(reply!({
      "token": token, "user": user,
    }))
}

pub async fn login(Json(body): Json<LoginUser>,Extension(state): Extension<Arc<APIState>>,
) -> APIResult {
    body.validate()?;
    let ret = body.find_one(&state.conn).await?;
    let user = match ret {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user["password"].to_string()) {
        return Err(reject!("密码不正确"));
    }
    let id = user["id"].to_string();
    let username = user["username"].to_string();
    let user = body.login(id.clone(), &state.conn).await?;
    let token = generate_token(id.clone(), username.clone());
    Ok(reply!({
      "token": token, "user": user,
    }))
}
