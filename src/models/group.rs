use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

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
