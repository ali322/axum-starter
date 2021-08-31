use crate::repository::{dao::UserDao, vo::User, DBError, POOL};
use bcrypt::{hash, verify};
use chrono::{Local, NaiveDateTime};
use rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(length(min = 1, max = 100))]
    pub password: String,
    #[validate(must_match(other = "password", message = "密码不匹配"))]
    pub repeat_password: String,
    #[validate(email)]
    pub email: Option<String>,
    #[serde(default = "now")]
    pub last_logined_at: NaiveDateTime,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewUser {
    pub async fn exists(&self) -> Result<UserDao, DBError> {
        let w = POOL.new_wrapper().eq("username", self.username.clone());
        UserDao::find_one(&w).await
    }
    pub async fn create(&self) -> Result<User, DBError> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(&self.password, 4).unwrap();
        let dao = UserDao {
            id: id.clone(),
            username: self.username.clone(),
            password: hashed_password,
            email: self.email.clone(),
            last_logined_at: now(),
            created_at: now(),
        };
        POOL.save(&dao, &[]).await?;
        User::find_one(id).await
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
}

impl UpdateUser {
    pub async fn save(&self, id: String) -> Result<User, DBError> {
        let mut dao: UserDao = POOL.fetch_by_column("id", &id).await?;
        dao.email = self.email.clone();
        POOL.update_by_column::<UserDao>("id", &mut dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(length(min = 1, max = 200))]
    pub username_or_email: String,
    #[validate(length(min = 3, max = 100))]
    pub password: String,
}

impl LoginUser {
    pub async fn find_one(&self) -> Result<User, DBError> {
        let w = POOL
            .new_wrapper()
            .eq("username", self.username_or_email.clone())
            .or()
            .eq("email", self.username_or_email.clone());
        UserDao::find_one(&w).await.map(Into::into)
    }
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(&self.password, target).unwrap()
    }
    pub async fn login(&self, id: String) -> Result<User, DBError> {
        let mut dao: UserDao = POOL.fetch_by_column("id", &id).await?;
        dao.last_logined_at = now();
        POOL.update_by_column("id", &mut dao).await?;
        Ok(dao.into())
    }
}
