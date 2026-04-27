use surrealdb::types::RecordId;
use crate::error::ApiError;
use crate::models::db::Database;
use crate::models::task::Task;

#[derive(Clone)]
pub struct TaskRepository {
    db: Database,
    table: &'static str
}

impl TaskRepository {
    pub fn new(db: Database) -> Self {
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
        self.db.get(id).await
    }

    async fn list(&self) -> Result<Vec<Task>, ApiError> {
        self.db.list(self.table).await
    }

    async fn create(&self, task: Task) -> Result<Task, ApiError> {
        self.db.create(self.table, task).await
    }
}
