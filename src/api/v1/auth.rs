use std::{sync::Arc};

use crate::{api::v1::APIState, lib::APIResult, repository::user::*};
use axum::{
    extract::{Extension, Json},
};
use validator::Validate;

pub async fn register(
    Json(body): Json<NewUser>,
    Extension(state): Extension<Arc<APIState>>,
) -> APIResult {
    body.validate()?;
    let pool = state.pool.clone();
    if body.exists(&pool).await.is_ok() {
      return Err(reject!("用户已存在"));
    }
    let user = body.create(&pool).await?;
    Ok(reply!(user))
}

pub async fn login(
    Json(body): Json<LoginUser>,
    Extension(state): Extension<Arc<APIState>>,
) -> APIResult {
  body.validate()?;
  let pool = state.pool.clone();
  let user = match body.find_one(&pool).await {
      Ok(val) => val,
      Err(_) => return Err(reject!("用户不存在"))
  };
  if !body.is_password_matched(&user.password) {
    return Err(reject!("密码不正确"));
  }
  let user = body.login(user.id, &pool).await?;
  Ok(reply!(user))
}