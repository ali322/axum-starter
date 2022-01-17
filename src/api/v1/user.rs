use crate::{
    repository::{
        dao::User,
        dto::{ChangePassword, QueryUser, ResetPassword, UpdateUser},
        vo, Dao,
    },
    util::{jwt::Auth, APIResult},
};
use axum::{
    extract::{Extension, Path, Query},
    routing::{get, post, put},
    Json, Router,
};
use validator::Validate;

async fn all(Query(q): Query<QueryUser>) -> APIResult {
    q.validate()?;
    let all = q.find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one: vo::User = match User::find_by_id(&id).await? {
        Some(val) => val.into(),
        None => return Err(reject!("用户不存在")),
    };
    Ok(reply!(one))
}

async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>) -> APIResult {
    body.validate()?;
    let mut one= match User::find_by_id(&id).await? {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    let updated: vo::User = body.save(&mut one).await?.into();
    Ok(reply!(updated))
}

async fn change_password(
    Json(body): Json<ChangePassword>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    body.validate()?;
    let user = match User::find_by_id(&auth.id).await? {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user.password) {
        return Err(reject!("旧密码不正确"));
    }
    let user = body.change_password(&user).await?;
    Ok(reply!(user))
}

async fn reset_password(
    Path(id): Path<String>,
    Json(body): Json<ResetPassword>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    if !auth.is_admin {
        return Err(reject!("仅管理员可访问"));
    }
    body.validate()?;
    let user = match User::find_by_id(&id).await? {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    let user = body.reset_password(&user).await?;
    Ok(reply!(user))
}

async fn me(Extension(auth): Extension<Auth>) -> APIResult {
    let user = match User::find_by_id(&auth.id).await? {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    Ok(reply!(user))
}

pub fn apply_routes() -> Router {
    let router = Router::new();
    router
        .route("/public/user", get(all))
        .route("/public/user/:id", get(one))
        .route("/user/:id", put(update))
        .route("/change/password", post(change_password))
        .route("/reset/:id/password", post(reset_password))
        .route("/me", get(me))
}
