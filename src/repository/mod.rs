use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;

pub mod user_repository;
pub mod task_repository;
pub mod group_repository;
use crate::error::ApiError;

pub trait Repository {
    type Record: Serialize + DeserializeOwned + Send + Sync;

    fn new(db: Arc<Surreal<Db>>) -> Self;
    async fn get(&self, id: RecordId) -> Result<Option<Self::Record>, ApiError>;
    async fn list(&self) -> Result<Vec<Self::Record>, ApiError>;
    async fn create(&self, record: Self::Record) -> Result<Self::Record, ApiError>;
}
