use super::Service;
use crate::repository::group_repository::GroupRepo;
use crate::models::group::{GroupDraft, GroupView, Group};
use crate::error::ApiError;
use surrealdb::types::RecordId;

#[derive(Clone)]
pub struct GroupService<G>
where
    G: GroupRepo
{
    pub repository: G
}

impl<G> GroupService<G>
where
    G: GroupRepo
{
    pub fn new(repository: G) -> Self {
        Self {
            repository
        }
    }
}

impl<G> Service for GroupService<G>
where
    G: GroupRepo
{
    type View = GroupView;
    type Draft = GroupDraft;
    type Id = RecordId;

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError> {
        let group = self.repository
            .get(id)
            .await?
            .map(GroupView::from);

        Ok(group)
    }

    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError> {
        let groups = self.repository
            .list()
            .await?
            .into_iter()
            .map(GroupView::from)
            .collect();

        Ok(groups)
    }

    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError> {
        let group = Group::from(draft);
        let group = self.repository.create(group).await?;

        Ok(GroupView::from(group))
    }
}

