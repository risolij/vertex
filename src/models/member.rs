use serde::{Deserialize, Serialize};
use surrealdb::types::{SurrealValue, RecordId};

use crate::{error::ApiError, repository::{Repository, member_repository::MemberRepository}, service::{MemberService, Service}};

#[derive(SurrealValue, Deserialize, Serialize)]
pub struct Member {
    id: Option<RecordId>,
    user: RecordId,
    group: RecordId 
}

#[derive(Deserialize)]
pub struct MemberDraft {
    user: RecordId,
    group: RecordId 
}

#[derive(Serialize)]
pub struct MemberView {
    id: RecordId,
    user: RecordId,
    group: RecordId
}

impl From<MemberDraft> for Member {
    fn from(draft: MemberDraft) -> Self {
        Self {
            id: None,
            user: draft.user,
            group: draft.group
        }
    }
}

impl TryFrom<Member> for MemberView {
    type Error = ApiError;

    fn try_from(member: Member) -> Result<Self, Self::Error> {
        Ok(Self {
            id: member.id.ok_or(ApiError::UnprocessableId)?,
            user: member.user,
            group: member.group
        })
    }
}

impl Service for MemberService<MemberRepository> {
    type Id = RecordId;
    type View = MemberView;
    type Draft = MemberDraft;

    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError> {
        self.repository
            .list()
            .await?
            .into_iter()
            .map(MemberView::try_from)
            .collect()
    }

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError> {
        self.repository
            .get(id)
            .await?
            .map(MemberView::try_from)
            .transpose()
    }

    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError> {
        let member = Member::from(draft);
        let member = self.repository.create(member).await?;

        MemberView::try_from(member)
    }
}
