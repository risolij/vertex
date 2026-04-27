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

impl Repository for MemberRepository {
    type Record = Member;
    type Id = RecordId;

    async fn get(&self, id: Self::Id) -> Result<Option<Self::Record>, ApiError> {
        todo!();
    }

    async fn list(&self) -> Result<Vec<Self::Record>, ApiError> {
        todo!();
    }

    async fn create(&self, record: Self::Record) -> Result<Self::Record, ApiError> {
        todo!();
    }
}


