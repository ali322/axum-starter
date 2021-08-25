use std::sync::Arc;

use crate::{
    repository::user::{User, UpdateUser},
    util::APIResult,
};
use super::APIState;
use axum::extract::{Extension, Json, Path};
use validator::Validate;

pub async fn all(Extension(state): Extension<Arc<APIState>>) -> APIResult {
    let users = User::find_all(&state.conn).await?;
    Ok(reply!(users))
}

pub async fn one(Path(id): Path<String>, Extension(state): Extension<Arc<APIState>>) -> APIResult {
    let user = User::find_one(id, &state.conn).await?;
    Ok(reply!(user))
}

pub async fn update(Path(id): Path<String>, Json(body): Json<UpdateUser>,  Extension(state): Extension<Arc<APIState>>) -> APIResult {
    body.validate()?;
    let updated = body.save(id, &state.conn).await?;
    Ok(reply!(updated))
}
