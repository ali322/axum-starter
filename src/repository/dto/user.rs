use crate::repository::{dao::User, DBError, POOL, Dao};
use bcrypt::{hash, verify};
use chrono::{Local, NaiveDateTime};
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
    pub avatar: Option<String>,
    pub memo: Option<String>,
    pub sys_role: Option<String>,
    #[serde(default = "now")]
    pub last_logined_at: NaiveDateTime,
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl NewUser {
    pub async fn create(self) -> Result<User, DBError> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(&self.password, 4).unwrap();
        let dao = User {
            id: id.clone(),
            username: self.username,
            password: hashed_password,
            email: self.email,
            last_logined_at: now(),
            created_at: now(),
        };
        User::create_one(&dao).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
}

impl UpdateUser {
    pub async fn save(self, id: &str) -> Result<User, DBError> {
        let w = POOL.new_wrapper().eq("id", id);
        let mut dao = User::find_one(&w).await?;
        dao.email = self.email;
        User::update_one(&dao, &w).await?;
        Ok(dao.into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(length(min = 1, max = 200))]
    pub username_or_email: String,
    #[validate(length(min = 3, max = 100))]
    pub password: String,
}

impl LoginUser {
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(self.password.clone(), target).unwrap()
    }
    pub async fn login(&self, dao: &User) -> Result<User, DBError> {
        let mut dao = dao.to_owned();
        dao.last_logined_at = now();
        let w = POOL.new_wrapper().eq("id", dao.id.clone());
        User::update_one(&dao, &w).await?;
        // POOL.save(&dao, &[]).await?;
        Ok(dao.into())
    }
}
