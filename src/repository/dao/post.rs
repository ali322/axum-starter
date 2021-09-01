use crate::repository::{DBError, POOL};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "posts")]
#[derive(Debug, Clone)]
pub struct PostDao {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PostDao {
    pub async fn find_one(w: &Wrapper) -> Result<Self, DBError> {
        let w = w.clone().order_by(true, &["id"]).limit(1);
        POOL.fetch_by_wrapper::<Self>(&w).await
    }
    pub async fn find_list(w: &Wrapper) -> Result<Vec<Self>, DBError> {
        POOL.fetch_list_by_wrapper(w).await
    }
    pub async fn create_one(&self) -> Result<i64, DBError> {
        let ret = POOL.save(&self, &[]).await?;
        Ok(ret.last_insert_id.unwrap())
    }
    pub async fn update_one(&self, w: &Wrapper) -> Result<u64, DBError> {
        POOL.update_by_wrapper(&self, w, &[]).await
    }
    pub async fn delete_one(w: &Wrapper) -> Result<u64, DBError> {
        POOL.remove_by_wrapper::<Self>(w).await
    }
}
