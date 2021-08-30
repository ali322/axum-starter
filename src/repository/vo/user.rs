use super::super::dao::UserDao;
use crate::util::datetime_format::naive_datetime;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: Option<String>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub last_logined_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl From<UserDao> for User {
    fn from(u: UserDao) -> Self {
        Self {
            id: u.id,
            username: u.username,
            password: u.password,
            email: u.email,
            last_logined_at: u.last_logined_at,
            created_at: u.created_at,
        }
    }
}
