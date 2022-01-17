use crate::{
    repository::{
        dao::Post,
        dto::{NewPost, UpdatePost},
        Dao,
    },
    util::{jwt::Auth, APIResult},
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post, put},
    Json, Router,
};
use validator::Validate;

async fn all() -> APIResult {
    let all = Post::find_all(None).await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<i32>) -> APIResult {
    let one = match Post::find_by_id(&id).await? {
        Some(val) => val,
        None => return Err(reject!("文章不存在")),
    };
    Ok(reply!(one))
}

async fn create(Json(mut body): Json<NewPost>, Extension(auth): Extension<Auth>) -> APIResult {
    body.validate()?;
    body.user_id = auth.id;
    let created = body.create().await?;
    Ok(reply!(created))
}

async fn update(Path(id): Path<i32>, Json(body): Json<UpdatePost>) -> APIResult {
    body.validate()?;
    let mut one = match Post::find_by_id(&id).await? {
      Some(val) => val,
      None => return Err(reject!("文章不存在")),
    };
    let updated = body.save(&mut one).await?;
    Ok(reply!(updated))
}

pub fn apply_routes() -> Router {
    let router = Router::new();
    router
        .route("/public/post", get(all))
        .route("/public/post/:id", get(one))
        .route("/post", post(create))
        .route("/post/:id", put(update))
}
