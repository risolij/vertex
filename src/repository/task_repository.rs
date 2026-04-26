use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;
use crate::models::task::{Task, TaskDraft};
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

    async fn get(&self, id: RecordId) -> Option<Self::Record> {
        self.db
            .select(id)
            .await
            .unwrap_or(None)
    }

    async fn list(&self) -> Vec<Self::Record> {
        self.db
            .select(self.table)
            .await
            .expect("Failed to execute select query")
    }

    async fn create(&self, task: Self::Record) -> Self::Record {
        let record: Self::Record = self.db
            .create(self.table)
            .content(task)
            .await
            .expect("Database communication failed")
            .expect("Template mismatch");

        record
    }
}
