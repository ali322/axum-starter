use crate::{
    repository::{
        dao::User,
        dto::{ChangePassword, QueryUser, ResetPassword, UpdateUser},
        vo, Dao,
    },
    util::{jwt::Auth, restrict::Restrict, APIResult},
};
use axum::{
    extract::{Extension, Path, Query},
    handler::{get, post, put},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;
use validator::Validate;

async fn all(Query(q): Query<QueryUser>) -> APIResult {
    q.validate()?;
    let all = q.find_all().await?;
    tracing::info!("all {:?}", all);
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one: vo::User = User::find_by_id(&id).await?.into();
    Ok(reply!(one))
}

async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>) -> APIResult {
    body.validate()?;
    let updated: vo::User = body.save(&id).await?.into();
    Ok(reply!(updated))
}

async fn change_password(
    Json(body): Json<ChangePassword>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    body.validate()?;
    let user = match User::find_by_id(&auth.id).await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&user.password) {
        return Err(reject!("旧密码不正确"));
    }
    let user = body.change_password(&user).await?;
    Ok(reply!(user))
}

async fn reset_password(
    id: String,
    Json(body): Json<ResetPassword>,
    Extension(auth): Extension<Auth>,
) -> APIResult {
    if !auth.is_admin {
        return Err(reject!("仅管理员可访问"));
    }
    body.validate()?;
    let user = match User::find_by_id(&id).await {
        Ok(val) => val,
        Err(_) => return Err(reject!("用户不存在")),
    };
    let user = body.reset_password(&user).await?;
    Ok(reply!(user))
}

async fn me(Extension(auth): Extension<Auth>) -> APIResult {
    let user = User::find_by_id(auth.id).await?;
    Ok(reply!(user))
}

pub fn apply_routes() -> Router<BoxRoute> {
    let router = Router::new();
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    router.route("/user", get(all))
        .route("/user/:id", put(update).get(one))
        .route("/change/password", post(change_password))
        .route("/reset/:id/password", post(reset_password))
        .route("/me", get(me))
        .layer(restrict_layer)
        .boxed()
}
