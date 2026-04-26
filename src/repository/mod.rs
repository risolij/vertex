use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;

pub mod user_repository;
pub mod task_repository;

pub trait Repository {
    type Record: Serialize + DeserializeOwned + Send + Sync;

    fn new(db: Arc<Surreal<Db>>) -> Self;
    async fn get(&self, id: RecordId) -> Option<Self::Record>;
    async fn list(&self) -> Vec<Self::Record>;
    async fn create(&self, record: Self::Record) -> Self::Record;
}
