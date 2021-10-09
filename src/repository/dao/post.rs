use crate::{repository::{DBError, Dao, POOL}, util::serde_format::{i32_bool, naive_datetime}};
use chrono::NaiveDateTime;
use app_macro::Dao;
use async_trait::async_trait;
use serde::Serialize;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "posts")]
#[derive(Debug, Clone, Dao)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub user_id: String,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
    #[serde(serialize_with = "i32_bool::serialize")]
    pub is_deleted: i32,
}
