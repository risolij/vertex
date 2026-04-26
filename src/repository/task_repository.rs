use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;
use crate::{error::ApiError, models::task::{Task, TaskDraft}};
use super::Repository;

#[derive(Clone)]
pub struct TaskRepository {
    db: Arc<Surreal<Db>>,
    table: &'static str
}

impl Repository for TaskRepository {
    type Record = Task;

    fn new(db: Arc<Surreal<Db>>) -> Self {
        Self {
            db,
            table: "task"
        }
    }

    async fn get(&self, id: RecordId) -> Result<Option<Self::Record>, ApiError> {
        let task = self.db
            .select(id)
            .await?;

        Ok(task)
    }

    async fn list(&self) -> Result<Vec<Self::Record>, ApiError> {
        let tasks = self.db
            .select(self.table)
            .await?;

        Ok(tasks)
    }

    async fn create(&self, task: Self::Record) -> Result<Self::Record, ApiError> {
        let task: Self::Record = self.db
            .create(self.table)
            .content(task)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(task)
    }
}
