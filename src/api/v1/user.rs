use crate::{
    repository::user::{UpdateUser, User},
    util::APIResult,
};
use axum::extract::{Json, Path};
use validator::Validate;

pub async fn all() -> APIResult {
    let users = User::find_all().await?;
    // Err(reject!("wrong"))
    Ok(reply!(users))
}

pub async fn one(Path(id): Path<String>) -> APIResult {
    let user = User::find_one(id).await?;
    Ok(reply!(user))
}

pub async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>) -> APIResult {
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}
