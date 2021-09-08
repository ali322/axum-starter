use crate::repository::{DBError, POOL, Dao};
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
