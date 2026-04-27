use serde::{Deserialize, Serialize};
use surrealdb::types::{SurrealValue, RecordId};

use crate::error::ApiError;

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
