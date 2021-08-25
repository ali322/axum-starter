use super::{DBConn, DBError};
use sea_orm::prelude::*;
use serde_json::Value as JsonValue;
use crate::{entity::users};

pub struct User;

impl User {
    pub async fn find_one(id: String, conn: &DBConn) -> Result<Option<JsonValue>, DBError> {
        users::Entity::find_by_id(id).into_json().one(conn).await
    }
    pub async fn find_all(conn: &DBConn) -> Result<Vec<JsonValue>, DBError> {
        users::Entity::find().into_json().all(conn).await
    }
}

// #[derive(Debug, Deserialize, Serialize, Validate)]
// pub struct NewUser {
//     #[validate(length(min = 1, max = 50))]
//     pub username: String,
//     #[validate(length(min = 1, max = 100))]
//     pub password: String,
//     #[validate(must_match(other = "password", message = "密码不匹配"))]
//     pub repeat_password: String,
//     #[validate(email)]
//     pub email: Option<String>,
//     #[serde(default = "now")]
//     pub last_logined_at: NaiveDateTime,
// }

// fn now() -> NaiveDateTime {
//     Local::now().naive_local()
// }

// impl NewUser {
//     pub async fn exists(&self) -> Result<User, DBError> {
//         let pool = super::POOL.clone();
//         query_as::<_, User>(
//             r"SELECT `id`, `username`, `password`,`email`,`last_logined_at`,`created_at` 
//                 FROM `users` WHERE `username` = ?",
//         )
//         .bind(self.username.clone())
//         .fetch_one(&pool)
//         .await
//     }
//     pub async fn create(&self) -> Result<User, DBError> {
//         let pool = super::POOL.clone();
//         let id = Uuid::new_v4().to_string();
//         let hashed_password = hash(&self.password, 4).unwrap();
//         query(
//             r"INSERT INTO `users`(`id`, `username`, `password`, `email`, `created_at`, `last_logined_at`) 
//             VALUES(?,?,?,?,?,?)"
//         ).bind(id.clone()).bind(self.username.clone()).bind(hashed_password).bind(self.email.clone()).bind(now()).bind(now())
//         .execute(&pool)
//         .await?;
//         User::find_one(id).await
//     }
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct UpdateUser {
//     #[validate(email)]
//     pub email: Option<String>,
// }

// impl UpdateUser {
//     pub async fn save(&self, id: String) -> Result<User, DBError> {
//         let pool = super::POOL.clone();
//         query("UPDATE `users` SET email = ? WHERE `id` = ?")
//             .bind(self.email.clone())
//             .bind(id.clone())
//             .execute(&pool)
//             .await?;
//         User::find_one(id).await
//     }
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct LoginUser {
//     #[validate(length(min = 1, max = 200))]
//     pub username_or_email: String,
//     #[validate(length(min = 3, max = 100))]
//     pub password: String,
// }

// impl LoginUser {
//     pub async fn find_one(&self) -> Result<User, DBError> {
//         let pool = super::POOL.clone();
//         query_as::<_, User>(
//             r"SELECT `id`, `username`, `password`,`email`,`last_logined_at`,`created_at` 
//     FROM `users` WHERE `username` = ? OR `email` = ?",
//         )
//         .bind(self.username_or_email.clone())
//         .bind(self.username_or_email.clone())
//         .fetch_one(&pool)
//         .await
//     }
//     pub fn is_password_matched(&self, target: &str) -> bool {
//         verify(&self.password, target).unwrap()
//     }
//     pub async fn login(&self, id: String) -> Result<User, DBError> {
//         let pool = super::POOL.clone();
//         query("UPDATE `users` SET last_logined_at = ? WHERE `id` = ?")
//             .bind(now())
//             .bind(id.clone())
//             .execute(&pool)
//             .await?;
//         User::find_one(id).await
//     }
// }
