use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;
use crate::error::ApiError;
use crate::models::task::Task;

#[derive(Clone)]
pub struct TaskRepository {
    db: Arc<Surreal<Db>>,
    table: &'static str
}

impl TaskRepository {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self {
            db,
            table: "task"
        }
    }
}

pub trait TaskRepo {
    async fn get(&self, id: RecordId) -> Result<Option<Task>, ApiError>;
    async fn list(&self) -> Result<Vec<Task>, ApiError>;
    async fn create(&self, task: Task) -> Result<Task, ApiError>;
}

impl TaskRepo for TaskRepository {
    async fn get(&self, id: RecordId) -> Result<Option<Task>, ApiError> {
        let task = self.db
            .select(id)
            .await?;

        Ok(task)
        
    }

    async fn list(&self) -> Result<Vec<Task>, ApiError> {
        let tasks = self.db
            .select(self.table)
            .await?;

        Ok(tasks)
        
    }

    async fn create(&self, task: Task) -> Result<Task, ApiError> {
        let task = self.db
            .create(self.table)
            .content(task)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(task)
    }
}
