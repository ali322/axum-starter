use axum::{handler::post, routing::BoxRoute, Json, Router};
use validator::Validate;

use crate::{
    repository::{
        dto::{LoginUser, NewUser},
        dao::User,
    },
    util::{jwt, APIResult},
};

async fn register(Json(body): Json<NewUser>) -> APIResult {
    body.validate()?;
    if User::find_by_username(&body.username).await.is_ok() {
        return Err(reject!("用户已存在"));
    }
    let user = body.create().await?;
    let token = jwt::generate_token(user.clone().id, user.clone().username);
    Ok(reply!({
      "token": token, "user": user,
    }))
}

async fn login(Json(body): Json<LoginUser>) -> APIResult {
    body.validate()?;
    let user = match User::find_by_username_or_email(&body.username_or_email).await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user.password) {
        return Err(reject!("密码不正确"));
    }
    let user = body.login(&user).await?;
    let token = jwt::generate_token(user.clone().id, user.clone().username);
    Ok(reply!({
      "token": token, "user": user
    }))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    v1.route("/register", post(register))
        .route("/login", post(login))
        .boxed()
}
