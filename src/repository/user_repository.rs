use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;
use crate::{error::ApiError, models::user::{User, UserDraft}};
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

    async fn get(&self, id: RecordId) -> Result<Option<Self::Record>, ApiError> {
        let user = self.db
            .select(id)
            .await
            .unwrap_or(None);

        Ok(user)
    }

    async fn list(&self) -> Result<Vec<Self::Record>, ApiError> {
        let users = self.db
            .select(self.table)
            .await?;

        Ok(users)
    }

    async fn create(&self, user: Self::Record) -> Result<Self::Record, ApiError> {
        let user: Self::Record = self.db
            .create(self.table)
            .content(user)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(user)
    }
}
