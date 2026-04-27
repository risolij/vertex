use surrealdb::types::RecordId;

use crate::error::ApiError;
use crate::models::db::Database;
use crate::models::group::Group;

#[derive(Clone)]
pub struct GroupRepository {
    db: Database,
    table: &'static str
}

impl GroupRepository {
    pub fn new(db: Database) -> Self {
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
        self.db.get(id).await
    }

    async fn list(&self) -> Result<Vec<Group>, ApiError> {
        self.db.list(self.table).await
    }

    async fn create(&self, group: Group) -> Result<Group, ApiError> {
        self.db.create(self.table, group).await
    }
}
