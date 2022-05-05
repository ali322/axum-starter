use crate::{
    repository::{dao::Post, vo, DBError, Dao, POOL},
    util::now,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewPost {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub content: String,
    #[serde(skip_deserializing)]
    pub user_id: String,
}

impl NewPost {
    pub async fn create(&self) -> Result<Post, DBError> {
        let mut dao = Post {
            id: None,
            title: self.title.clone(),
            content: self.content.clone(),
            user_id: self.user_id.clone(),
            created_at: now(),
            updated_at: now(),
            is_deleted: 0,
        };
        let id = Post::create_one(&dao).await?;
        dao.id = Some(id as i32);
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub content: String,
}

impl UpdatePost {
    pub async fn save(&self, dao:&mut Post) -> Result<vo::Post, DBError> {
        dao.title = self.title.clone();
        dao.content = self.content.clone();
        let w = POOL.new_wrapper().eq("id", dao.id);
        Post::update_one(&dao, w).await?;
        Ok(dao.to_owned().into())
    }
}
