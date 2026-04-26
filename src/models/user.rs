use crate::service::UserService;
use crate::repository::Repository;
use crate::repository::user_repository::UserRepository;
use surrealdb::types::{RecordId, SurrealValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, SurrealValue)]
pub struct User {
    id: Option<RecordId>,
    name: String
}

#[derive(Deserialize, Serialize, SurrealValue)]
pub struct UserDraft {
    name: String,
}

impl From<UserDraft> for User {
    fn from(draft: UserDraft) -> Self {
        User {
            id: None,
            name: draft.name
        }
    }
}

impl UserService<UserRepository> {
    pub async fn get_user(&self, id: RecordId) -> Option<User> {
        self.repository.get(id).await
    }

    pub async fn get_users(&self) -> Vec<User> {
        self.repository.list().await
    }

    pub async fn create_user(&self, draft: UserDraft) -> User {
        let user = User::from(draft);
        self.repository.create(user).await
    }
}
