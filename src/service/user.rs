use super::Service;
use crate::repository::user_repository::UserRepo;
use crate::error::ApiError;
use crate::models::user::{UserView, UserDraft, User};
use surrealdb::types::RecordId;

#[derive(Clone)]
pub struct UserService<R>
where
    R: UserRepo
{
    pub repository: R
}

impl<R> UserService<R>
where
    R: UserRepo
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<U> Service for UserService<U>
where
    U: UserRepo
{
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
