use std::sync::Arc;
use surrealdb::{Surreal, engine::local::Db, types::RecordId};

use crate::models::member::Member;
use crate::error::ApiError;

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

pub trait MemberRepo {
    async fn get(&self, id: RecordId) -> Result<Option<Member>, ApiError>;
    async fn list(&self) -> Result<Vec<Member>, ApiError>;
    async fn create(&self, member: Member) -> Result<Member, ApiError>;
}

impl MemberRepo for MemberRepository {
    async fn get(&self, id: RecordId) -> Result<Option<Member>, ApiError> {
        let member = self.db
            .select(id)
            .await?;

        Ok(member)
    }

    async fn list(&self) -> Result<Vec<Member>, ApiError> {
        let members = self.db
            .select(self.table)
            .await?;

        Ok(members)
    }

    async fn create(&self, member: Member) -> Result<Member, ApiError> {
        let member = self.db
            .create(self.table)
            .content(member)
            .await?
            .ok_or(ApiError::InternalServerError)?;

        Ok(member)
    }
}
