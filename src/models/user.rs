use crate::error::ApiError;
use crate::service::{Service, UserService};
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

impl Service for UserService<UserRepository> {
    type View = UserView;
    type Draft = UserDraft;
    type Id = RecordId;

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError> {
        let view = self
            .repository
            .get(id)
            .await?
            .map(UserView::from);

        Ok(view)
    }

    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError> {
        let views = self.repository
            .list()
            .await?
            .into_iter()
            .map(UserView::from)
            .collect();

        Ok(views)
    }

    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError> {
        let user = User::from(draft);
        let user = self.repository.create(user).await?;

        Ok(UserView::from(user))
    }
}
