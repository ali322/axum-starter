use crate::{
    repository::user::{ UserDao, UpdateUser},
    util::APIResult,
};
use axum::extract::{Json, Path};
use validator::Validate;

pub async fn all() -> APIResult {
    let users = UserDao::find_all().await?;
    Ok(reply!(users))
}

pub async fn one(Path(id): Path<String>) -> APIResult {
    let user = UserDao::find_one(id).await?;
    Ok(reply!(user))
}

pub async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>) -> APIResult {
    body.validate()?;
    let updated = body.save(id).await?;
    Ok(reply!(updated))
}
