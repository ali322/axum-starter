use crate::repository::{dao::PostDao, vo::Post, DBError, POOL};
use chrono::{Local, NaiveDateTime};
use rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewPost {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub content: String,
    pub user_id: String,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewPost {
    pub async fn create(&self) -> Result<Post, DBError> {
        let dao = PostDao {
            id: None,
            title: self.title.clone(),
            content: self.content.clone(),
            user_id: self.user_id.clone(),
            created_at: now(),
            updated_at: now(),
        };
        PostDao::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub content: String,
}

impl UpdatePost {
    pub async fn save(&self, id: i32) -> Result<Post, DBError> {
        let mut dao: PostDao = POOL.fetch_by_column("id", &id).await?;
        dao.title = self.title.clone();
        dao.content = self.content.clone();
        let w = POOL.new_wrapper().eq("id", id);
        PostDao::update_one(&dao, &w).await?;
        // POOL.save(&dao, &[]).await?;
        Ok(dao.into())
    }
}
