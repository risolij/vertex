use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::RecordId;
use crate::error::ApiError;
use crate::models::user::User;

#[derive(Clone)]
pub struct UserRepository {
    db: Arc<Surreal<Db>>,
    table: &'static str
}

impl UserRepository {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
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
        let user = self.db
            .select(id)
            .await?;

        Ok(user)
        
    }

    async fn list(&self) -> Result<Vec<User>, ApiError> {
        let users = self.db
            .select(self.table)
            .await?;

        Ok(users)
    }

    async fn create(&self, user: User) -> Result<User, ApiError> {
        let user = self.db
            .create(self.table)
            .content(user)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(user)
    }
}
