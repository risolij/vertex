use std::sync::Arc;
use surrealdb::{Surreal, engine::local::Db, types::RecordId};

use crate::models::db::Database;
use crate::models::member::Member;
use crate::error::ApiError;

#[derive(Clone)]
pub struct MemberRepository {
    db: Database,
    table: &'static str
}

impl MemberRepository {
    pub fn new(db: Database) -> Self {
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
        self.db.get(id).await
    }

    async fn list(&self) -> Result<Vec<Member>, ApiError> {
        self.db.list(self.table).await
    }

    async fn create(&self, member: Member) -> Result<Member, ApiError> {
        self.db.create(self.table, member).await
    }
}
