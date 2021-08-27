use super::{DBError, POOL};
use crate::util::datetime_format::naive_datetime;
use bcrypt::{hash, verify};
use chrono::{Local, NaiveDateTime};
use rbatis::{crud::CRUD, wrapper::Wrapper};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: Option<String>,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub last_logined_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl From<UserDao> for User {
    fn from(u: UserDao) -> Self {
        Self {
            id: u.id,
            username: u.username,
            password: u.password,
            email: u.email,
            last_logined_at: u.last_logined_at,
            created_at: u.created_at,
        }
    }
}

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
        UserDao::find_one_by_wrapper(&w).await
    }
    pub async fn create(&self) -> Result<User, DBError> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(&self.password, 4).unwrap();
        let u = UserDao {
            id: id.clone(),
            username: self.username.clone(),
            password: hashed_password,
            email: self.email.clone(),
            last_logined_at: now(),
            created_at: now(),
        };
        POOL.save(&u, &[]).await?;
        UserDao::find_one(id).await
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
}

impl UpdateUser {
    pub async fn save(&self, id: String) -> Result<User, DBError> {
        let mut u: UserDao = POOL.fetch_by_column("id", &id).await?;
        u.email = self.email.clone();
        POOL.update_by_column::<UserDao>("id", &mut u).await?;
        Ok(u.into())
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
        UserDao::find_one_by_wrapper(&w).await.map(Into::into)
    }
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(&self.password, target).unwrap()
    }
    pub async fn login(&self, id: String) -> Result<User, DBError> {
        let mut u: UserDao = POOL.fetch_by_column("id", &id).await?;
        u.last_logined_at = now();
        POOL.update_by_column("id", &mut u).await?;
        Ok(u.into())
    }
}
