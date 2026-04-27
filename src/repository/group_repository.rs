use std::sync::Arc;
use surrealdb::types::RecordId;
use surrealdb::{Surreal, engine::local::Db};

use super::Repository;
use crate::error::ApiError;
use crate::models::group::Group;

#[derive(Clone)]
pub struct GroupRepository {
    db: Arc<Surreal<Db>>,
    table: &'static str
}

impl GroupRepository {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self {
            db,
            table: "group"
        }
    }
}

impl Repository for GroupRepository {
    type Record = Group;
    type Id = RecordId;

    async fn get(&self, id: Self::Id) -> Result<Option<Self::Record>, ApiError> {
        let group= self.db
            .select(id)
            .await?;

        Ok(group)
    }

    async fn list(&self) -> Result<Vec<Self::Record>, ApiError> {
        let groups= self.db
            .select(self.table)
            .await?;

        Ok(groups)
    }

    async fn create(&self, group: Self::Record) -> Result<Self::Record, ApiError> {
        let group: Self::Record = self.db
            .create(self.table)
            .content(group)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(group)
    }
}
