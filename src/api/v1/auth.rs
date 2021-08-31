use crate::{
    repository::dto::*,
    util::{jwt::generate_token, APIResult},
};
use axum::{handler::post, routing::BoxRoute, Json, Router};
use validator::Validate;

async fn register(Json(body): Json<NewUser>) -> APIResult {
    body.validate()?;
    if body.exists().await.is_ok() {
        return Err(reject!("用户已存在"));
    }
    let user = body.create().await?;
    let token = generate_token(user.clone().id, user.clone().username);
    Ok(reply!({
      "token": token, "user": user,
    }))
}

async fn login(Json(body): Json<LoginUser>) -> APIResult {
    body.validate()?;
    let user = match body.find_one().await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user.password) {
        return Err(reject!("密码不正确"));
    }
    let user = body.login(user.id).await?;
    let token = generate_token(user.clone().id, user.clone().username);
    Ok(reply!({
      "token": token, "user": user,
    }))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/register", post(register))
        .route("/login", post(login))
        .boxed()
}
