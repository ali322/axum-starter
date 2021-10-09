use crate::{repository::dao, util::serde_format::{naive_datetime, i32_bool}};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    id: Option<i32>,
    title: String,
    content: String,
    #[serde(serialize_with = "naive_datetime::serialize")]
    created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    updated_at: NaiveDateTime,
    #[serde(serialize_with = "i32_bool::serialize")]
    pub is_deleted: i32,
}

impl From<dao::Post> for Post {
    fn from(d: dao::Post) -> Self {
        Self {
            id: d.id,
            title: d.title,
            content: d.content,
            created_at: d.created_at,
            updated_at: d.updated_at,
            is_deleted: d.is_deleted,
        }
    }
}
