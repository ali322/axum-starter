use crate::repository::{DBError, POOL};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "users")]
#[derive(Debug, Clone)]
pub struct UserDao {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
    pub sys_role: Option<String>,
    pub is_actived: Option<i32>,
    pub last_logined_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl UserDao {
    pub async fn find_one(w: &Wrapper) -> Result<Self, DBError> {
        let w = w.clone().order_by(true, &["id"]).limit(1);
        POOL.fetch_by_wrapper::<Self>(&w).await
    }
    pub async fn find_list(w: &Wrapper) -> Result<Vec<Self>, DBError> {
        POOL.fetch_list_by_wrapper::<Self>(w).await
    }
    pub async fn create_one(&self) -> Result<i64, DBError> {
        let created = POOL.save(&self, &[]).await?;
        Ok(created.last_insert_id.unwrap())
    }
    pub async fn update_one(&self, w: &Wrapper) -> Result<u64, DBError> {
        POOL.update_by_wrapper(&self, w, &[]).await
    }
}
