use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use crate::repository::group_repository::GroupRepo;
use crate::service::{GroupService, Service};
use crate::error::ApiError;

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Group {
    id: Option<RecordId>,
    number: String,
    name: String,
    active: bool
}

#[derive(Serialize)]
pub struct GroupView {
    id: RecordId,
    number: String,
    name: String,
    active: bool
}

impl From<Group> for GroupView {
    fn from(group: Group) -> Self {
        Self {
            id: group.id.unwrap(),
            number: group.number,
            name: group.name,
            active: group.active
        }
    }
}

#[derive(Deserialize)]
pub struct GroupDraft {
    number: Option<String>,
    name: Option<String>
}

impl From<GroupDraft> for Group {
    fn from(draft: GroupDraft) -> Self {
        Self {
            id: None,
            number: draft.number.unwrap(),
            name: draft.name.unwrap(),
            active: true
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
