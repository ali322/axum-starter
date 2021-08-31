use crate::{
    repository::{dao::PostDao, vo::User, DBError, POOL},
    util::datetime_format::naive_datetime,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub user: Option<User>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub updated_at: NaiveDateTime,
}

impl From<PostDao> for Post {
    fn from(v: PostDao) -> Self {
        Self {
            id: v.id,
            title: v.title,
            content: v.content,
            user: None,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

impl Post {
    pub async fn find_one(id: i32) -> Result<Self, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        PostDao::find_one(&w).await.map(Into::into)
    }
    pub async fn find_all() -> Result<Vec<Self>, DBError> {
        let w = POOL.new_wrapper();
        let all = PostDao::find_list(&w).await?;
        let all: Vec<Self> = all.iter().map(|v| v.clone().into()).collect();
        Ok(all)
    }
}
