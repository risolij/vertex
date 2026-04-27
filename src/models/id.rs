use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;

use crate::error::ApiError;

#[derive(Deserialize, Serialize)]
pub struct Id(String);

impl TryFrom<Id> for RecordId {
    type Error = ApiError;

    fn try_from(id: Id) -> Result<Self, Self::Error> {
        let record_id = RecordId::parse_simple(&id.0)
            .map_err(|_| ApiError::UnprocessableId)?;

        Ok(record_id)
    }
}
