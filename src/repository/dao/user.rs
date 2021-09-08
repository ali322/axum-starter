use crate::{repository::{DBError, POOL, Dao}, util::datetime_format::naive_datetime,};
use chrono::NaiveDateTime;
use app_macro::Dao;
use async_trait::async_trait;
use serde::Serialize;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "users")]
#[derive(Debug, Clone, Dao)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    #[serde(serialize_with="naive_datetime::serialize")]
    pub last_logined_at: NaiveDateTime,
    #[serde(serialize_with="naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl User {
  pub async fn find_by_username(username: &str) -> Result<Self, DBError> {
      let w = POOL.new_wrapper().eq("username", username);
      Self::find_one(&w).await
  }
  pub async fn find_by_username_or_email(username_or_email: &str) -> Result<Self, DBError> {
      let w = POOL
          .new_wrapper()
          .eq("username", username_or_email)
          .or()
          .eq("email", username_or_email);
      Self::find_one(&w).await
  }
}
