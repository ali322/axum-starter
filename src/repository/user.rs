use super::{DBConn, DBError};
use crate::entity::users;
use bcrypt::{hash, verify};
use chrono::{Local, NaiveDateTime};
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use validator::Validate;

pub struct User;

impl User {
    pub async fn find_one(id: String, conn: &DBConn) -> Result<Option<JsonValue>, DBError> {
        users::Entity::find_by_id(id).into_json().one(conn).await
    }
    pub async fn find_all(conn: &DBConn) -> Result<Vec<JsonValue>, DBError> {
        users::Entity::find().into_json().all(conn).await
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
    pub async fn exists(&self, conn: &DBConn) -> Result<Option<JsonValue>, DBError> {
        users::Entity::find()
            .filter(users::Column::Username.eq(self.username.as_str()))
            .into_json()
            .one(conn)
            .await
    }
    pub async fn create(&self, conn: &DBConn) -> Result<Option<JsonValue>, DBError> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(&self.password, 4).unwrap();
        let user = users::ActiveModel {
            id: Set(id.to_owned()),
            username: Set(self.username.clone()),
            password: Set(hashed_password),
            email: Set(self.email.clone()),
            last_logined_at: Set(now()),
            created_at: Set(now()),
        };
        let ret = users::Entity::insert(user).exec(conn).await?;
        users::Entity::find_by_id(ret.last_insert_id)
            .into_json()
            .one(conn)
            .await
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
}

impl UpdateUser {
    pub async fn save(&self, id: String, conn: &DBConn) -> Result<Option<JsonValue>, DBError> {
        let ret = users::Entity::find_by_id(id.as_str()).one(conn).await?;
        if let Some(user) = ret {
            let mut user: users::ActiveModel = user.into();
            user.email = Set(self.email.clone());
            users::Entity::update(user).exec(conn).await?;
            User::find_one(id, conn).await
        } else {
            Ok(None)
        }
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
    pub async fn find_one(&self, conn: &DBConn) -> Result<Option<JsonValue>, DBError> {
        users::Entity::find().filter(
            Condition::any()
                .add(users::Column::Username.eq(self.username_or_email.as_str()))
                .add(users::Column::Email.eq(self.username_or_email.as_str())),
        ).into_json().one(conn).await
    }
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(&self.password, target).unwrap()
    }
    pub async fn login(&self, id: String, conn: &DBConn) -> Result<Option<JsonValue>, DBError> {
      let ret = users::Entity::find_by_id(id.as_str()).one(conn).await?;
      if let Some(user) = ret {
          let mut user: users::ActiveModel = user.into();
          user.last_logined_at = Set(now());
          users::Entity::update(user).exec(conn).await?;
          User::find_one(id, conn).await
      } else {
          Ok(None)
      }
    }
}
