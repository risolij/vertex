use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;
use crate::models::user::{User, UserDraft};
use super::Repository;

#[derive(Clone)]
pub struct UserRepository {
    db: Arc<Surreal<Db>>,
    table: &'static str
}

impl Repository for UserRepository {
    type Record = User;

    fn new(db: Arc<Surreal<Db>>) -> Self {
        Self {
            db,
            table: "user"
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

    async fn create(&self, user: Self::Record) -> Self::Record {
        let record: Self::Record = self.db
            .create(self.table)
            .content(user)
            .await
            .expect("Database communication failed")
            .expect("Template mismatch");

        record
    }
}
