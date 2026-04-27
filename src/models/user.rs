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

#[derive(Serialize, SurrealValue)]
pub struct UserView {
    name: String
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        Self {
            name: user.name
        }
    }
}
