use std::sync::Arc;

use surrealdb::{Surreal, engine::local::Db, types::{RecordId, SurrealValue}};

use crate::error::ApiError;

#[derive(Clone)]
pub struct Database {
    db: Arc<Surreal<Db>>
}

impl Database {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self { db }
    }

    pub async fn get<E>(&self, id: RecordId) -> Result<Option<E>, ApiError>
    where
        E: SurrealValue
    {
        Ok(self.db
            .select(id)
            .await?)
    }

    pub async fn list<E>(&self, table: &str) -> Result<Vec<E>, ApiError>
    where
        E: SurrealValue
    {
        Ok(self.db
            .select(table)
            .await?)
    }

    pub async fn create<E>(&self, table: &str, entity: E) -> Result<E, ApiError>
    where
        E: SurrealValue
    {
        self.db
            .create(table)
            .content(entity)
            .await?
            .ok_or(ApiError::InternalServerError)
    }
}
