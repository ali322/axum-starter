use rbatis::{
    core::{runtime::task::block_on, Error},
    plugin::logic_delete::RbatisLogicDeletePlugin,
    rbatis::Rbatis,
};
use std::env;

pub mod dao;
pub mod dto;
pub mod vo;
mod traits;

pub use traits::Dao;
pub type DBPool = Rbatis;
pub type DBError = Error;

async fn init_db() -> DBPool {
    let database_url =
        env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");
    let mut rbatis = Rbatis::new();
    rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt(
        "is_deleted",
        1,
        0,
    )));
    rbatis
        .link(&database_url)
        .await
        .expect("connect to database failed");
    rbatis
}

lazy_static! {
    static ref POOL: DBPool = block_on(async { init_db().await });
}
