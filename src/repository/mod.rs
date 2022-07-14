use once_cell::sync::Lazy;
use rbatis::{
    core::{Error},
    plugin::logic_delete::RbatisLogicDeletePlugin,
    rbatis::Rbatis, db::DBPoolOptions,
};
use std::env;

pub mod dao;
pub mod dto;
pub mod vo;
mod traits;

pub use traits::Dao;
pub type DBPool = Rbatis;
pub type DBError = Error;

pub async fn init_db() {
  let database_url =
  env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");
  let mut opt = DBPoolOptions::new();
  opt.max_connections = 10;
  POOL
      .link_opt(&database_url, opt)
      .await
      .expect("connect to database failed");
}

pub static POOL:Lazy<DBPool> = Lazy::new(||{
  let mut rb = Rbatis::new();
  rb.set_logic_plugin(RbatisLogicDeletePlugin::new_opt("is_deleted", 1, 0));
  rb
});

