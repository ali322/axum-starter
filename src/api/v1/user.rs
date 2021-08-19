use std::sync::Arc;

use crate::{api::v1::APIState, lib::APIResult, repository::user::{UpdateUser, User}};
use axum::{extract::{Extension, Json, Path}};
use validator::Validate;

pub async fn all(
    Extension(state): Extension<Arc<APIState>>,
) -> APIResult {
    let pool = state.pool.clone();
    let users = User::find_all(&pool).await?;
    // Err(reject!("wrong"))
    Ok(reply!(users))
}


pub async fn one(Path(id): Path<String>, Extension(state): Extension<Arc<APIState>>) -> APIResult{
  let pool = state.pool.clone();
  let user = User::find_one(id, &pool).await?;
  Ok(reply!(user))
}

pub async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>, Extension(state): Extension<Arc<APIState>>) -> APIResult{
  body.validate()?;
  let pool = state.pool.clone();
  let updated = body.save(id, &pool).await?;
  Ok(reply!(updated))
}