use async_trait::async_trait;
use rbatis::{core::Error, wrapper::Wrapper};
use serde::Serialize;

#[async_trait]
pub trait Dao: Sized {
    async fn find_one(w: &Wrapper) -> Result<Self, Error>;
    async fn find_by_id<T>(id: T) -> Result<Self, Error> where T: Serialize + Send + Sync;
    async fn find_list(w: &Wrapper) -> Result<Vec<Self>, Error>;
    async fn find_all() -> Result<Vec<Self>, Error>;
    async fn find_by_ids<T>(id: Vec<T>) -> Result<Vec<Self>, Error> where T: Serialize + Send + Sync;
    async fn create_one(&self) -> Result<i64, Error>;
    async fn update_one(&self, w: &Wrapper) -> Result<u64, Error>;
    async fn delete_one(w: &Wrapper) -> Result<u64, Error>;
    async fn delete_by_id<T>(id: T) -> Result<u64, Error> where T: Serialize + Send + Sync;
}