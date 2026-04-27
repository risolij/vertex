use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod user_repository;
pub mod task_repository;
pub mod group_repository;
use crate::error::ApiError;

pub trait Repository {
    type Record: Serialize + DeserializeOwned + Send + Sync;
    type Id;

    async fn get(&self, id: Self::Id) -> Result<Option<Self::Record>, ApiError>;
    async fn list(&self) -> Result<Vec<Self::Record>, ApiError>;
    async fn create(&self, record: Self::Record) -> Result<Self::Record, ApiError>;
}
