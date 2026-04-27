use std::sync::Arc;
use surrealdb::{Surreal, engine::local::Db, types::RecordId};

use crate::models::member::Member;
use crate::error::ApiError;
use super::Repository;


#[derive(Clone)]
pub struct MemberRepository {
    db: Arc<Surreal<Db>>,
    table: &'static str
}

impl MemberRepository {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self {
            db,
            table: "member"
        }
    }
}

impl Repository for MemberRepository {
    type Record = Member;
    type Id = RecordId;

    async fn get(&self, id: Self::Id) -> Result<Option<Self::Record>, ApiError> {
        let member = self.db
            .select(id)
            .await?;

        Ok(member)
    }

    async fn list(&self) -> Result<Vec<Self::Record>, ApiError> {
        let members = self.db
            .select(self.table)
            .await?;

        Ok(members)
    }

    async fn create(&self, member: Self::Record) -> Result<Self::Record, ApiError> {
        let member = self.db
            .create(self.table)
            .content(member)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(member)
    }
}
