use super::super::{vo::User, DBError, POOL};
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, wrapper::Wrapper};

#[crud_table(table_name: "users")]
#[derive(Debug, Clone)]
pub struct UserDao {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub last_logined_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl UserDao {
    pub async fn find_one_by_wrapper(w: &Wrapper) -> Result<Self, DBError> {
        let w = w.clone().order_by(true, &["id"]).limit(1);
        POOL.fetch_by_wrapper::<Self>(&w).await
    }
    pub async fn find_list_by_wrapper(w: &Wrapper) -> Result<Vec<Self>, DBError> {
        POOL.fetch_list_by_wrapper::<Self>(w).await
    }
    pub async fn find_one(id: String) -> Result<User, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        Self::find_one_by_wrapper(&w).await.map(Into::into)
    }
    pub async fn find_all() -> Result<Vec<User>, DBError> {
        let all = POOL.fetch_list::<UserDao>().await?;
        let all: Vec<User> = all.iter().map(|v| User::from(v.clone())).collect();
        Ok(all)
    }
}
