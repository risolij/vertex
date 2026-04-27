use surrealdb::types::RecordId;
use crate::error::ApiError;
use crate::models::db::Database;
use crate::models::user::User;

#[derive(Clone)]
pub struct UserRepository {
    db: Database,
    table: &'static str
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            table: "user"
        }
    }
}

pub trait UserRepo {
    async fn get(&self, id: RecordId) -> Result<Option<User>, ApiError>;
    async fn list(&self) -> Result<Vec<User>, ApiError>;
    async fn create(&self, user: User) -> Result<User, ApiError>;
}

impl UserRepo for UserRepository {
    async fn get(&self, id: RecordId) -> Result<Option<User>, ApiError> {
        self.db.get(id).await
    }

    async fn list(&self) -> Result<Vec<User>, ApiError> {
        self.db.list(self.table).await
    }

    async fn create(&self, user: User) -> Result<User, ApiError> {
        self.db.create(self.table, user).await
    }
}
