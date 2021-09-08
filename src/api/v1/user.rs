use crate::{
    repository::{dto::UpdateUser, dao::User, Dao},
    util::{restrict::Restrict, APIResult},
};
use axum::{
    extract::Path,
    handler::{get, put, Handler},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;
use validator::Validate;

async fn all() -> APIResult {
    let all = User::find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> APIResult {
    let one = User::find_by_id(&id).await?;
    Ok(reply!(one))
}

async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>) -> APIResult {
    body.validate()?;
    let updated = body.save(&id).await?;
    Ok(reply!(updated))
}

pub fn apply_routes(v1: Router<BoxRoute>) -> Router<BoxRoute> {
    // let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    v1.route("/user", get(all))
        .route("/user/:id", put(update).get(one))
        .boxed()
}
