use std::{env};
use sea_orm::{Database, DatabaseConnection, DbErr};

pub mod user;

pub type DBConn = DatabaseConnection;
pub type DBError = DbErr;

pub async fn init_db_conn() -> DBConn {
  let database_url =
      env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");
  Database::connect(&database_url).await.expect("failed to connect databse")
}