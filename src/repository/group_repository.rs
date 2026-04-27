use std::sync::Arc;
use surrealdb::types::RecordId;
use surrealdb::{Surreal, engine::local::Db};

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

pub trait GroupRepo {
    async fn get(&self, id: RecordId) -> Result<Option<Group>, ApiError>;
    async fn list(&self) -> Result<Vec<Group>, ApiError>;
    async fn create(&self, group: Group) -> Result<Group, ApiError>;
}

impl GroupRepo for GroupRepository {
    async fn get(&self, id: RecordId) -> Result<Option<Group>, ApiError> {
        let group= self.db
            .select(id)
            .await?;

        Ok(group)
    }

    async fn list(&self) -> Result<Vec<Group>, ApiError> {
        let groups= self.db
            .select(self.table)
            .await?;

        Ok(groups)
    }

    async fn create(&self, group: Group) -> Result<Group, ApiError> {
        let group = self.db
            .create(self.table)
            .content(group)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(group)
    }
}
