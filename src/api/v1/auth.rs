use axum::{routing::post, Json, Router};
use validator::Validate;

use crate::{
    repository::{
        dao::User,
        dto::{LoginUser, NewUser},
    },
    util::{
        jwt::{self, Auth},
        APIResult,
    },
};

async fn register(Json(body): Json<NewUser>) -> APIResult {
    body.validate()?;
    if User::find_by_username(&body.username).await.is_ok() {
        return Err(reject!("用户已存在"));
    }
    let user = body.create().await?;
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        is_admin: false,
    });
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
    if user.is_actived == 0 {
        return Err(reject!("用户被禁用"));
    }
    let user = body.login(&user).await?;
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        is_admin: true,
    });
    Ok(reply!({
      "token": token, "user": user
    }))
}

pub fn apply_routes() -> Router {
    let router = Router::new();
    router
        .route("/register", post(register))
        .route("/login", post(login))
}
