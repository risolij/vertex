use crate::service::Service;
use crate::repository::Repository;
use crate::repository::user_repository::UserRepository;
use surrealdb::types::{RecordId, SurrealValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, SurrealValue)]
pub struct User {
    id: RecordId,
    name: String
}

#[derive(Deserialize, Serialize, SurrealValue)]
pub struct UserDraft {
    name: String,
}

impl Service<UserRepository> {
    pub async fn get_user(&self, id: RecordId) -> Option<User> {
        self.repository.get(id).await
    }

    pub async fn get_users(&self) -> Vec<User> {
        self.repository.list().await
    }

    pub async fn create_user(&self, draft: UserDraft) -> User {
        self.repository.create(draft).await
    }
}
