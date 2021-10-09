use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::repository::dao;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
  id: String,
  username: String,
  email: Option<String>,
  created_at: NaiveDateTime,
  last_logined_at: NaiveDateTime,
}

impl From<dao::User> for User{
  fn from(d: dao::User) -> Self {
      Self{
        id: d.id,
        username: d.username,
        email: d.email,
        created_at: d.created_at,
        last_logined_at: d.last_logined_at
      }
  }
}