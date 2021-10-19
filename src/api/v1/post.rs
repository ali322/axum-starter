use crate::{
    repository::{
        dao::Post,
        dto::{NewPost, UpdatePost},
        Dao,
    },
    util::{restrict::Restrict, APIResult},
};
use axum::{
    extract::Path,
    handler::{get, Handler},
    routing::BoxRoute,
    Json, Router,
};
use tower_http::auth::RequireAuthorizationLayer;
use validator::Validate;

async fn all() -> APIResult {
    let all = Post::find_all(None).await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<i32>) -> APIResult {
    let one = Post::find_by_id(id).await?;
    Ok(reply!(one))
}

async fn create(Json(body): Json<NewPost>) -> APIResult {
    body.validate()?;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(Path(id): Path<i32>, Json(body): Json<UpdatePost>) -> APIResult {
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}

pub fn apply_routes() -> Router<BoxRoute> {
    let router = Router::new();
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    router.route("/post", get(all).post(create.layer(restrict_layer.clone())))
        .route(
            "/post/:id",
            get(one).put(update.layer(restrict_layer.clone())),
        )
        .layer(restrict_layer)
        .boxed()
}
