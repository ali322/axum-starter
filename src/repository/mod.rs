use sqlx::{
  mysql::{MySqlConnectOptions, MySqlPoolOptions},
  pool::Pool,
  ConnectOptions, MySql,
};
use std::time::Duration;
use std::{env, str::FromStr};
use tracing::log::LevelFilter;

pub mod user;

pub type DBPool = Pool<MySql>;
pub type DBError = sqlx::Error;

pub async fn init_db_pool() -> DBPool {
  let database_url =
      env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");
  let options = MySqlConnectOptions::from_str(&database_url)
      .expect("DATABASE_URL is invalid")
      .log_slow_statements(LevelFilter::Warn, Duration::from_secs(5))
      .log_statements(LevelFilter::Info)
      .clone();
  MySqlPoolOptions::new()
      .max_connections(5)
      .connect_with(options.clone())
      .await
      .expect("database pool could not create")
}

lazy_static!{
  static ref POOL:DBPool = tokio::runtime::Runtime::new().unwrap().block_on(async {
    init_db_pool().await
  });
}