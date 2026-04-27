use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::RecordId;
use crate::models::id::Id;
use crate::repository::group_repository::GroupRepository;
use crate::repository::member_repository::MemberRepository;
use crate::models::member::{MemberView, MemberDraft};
use crate::repository::user_repository::UserRepository;
use crate::service::{Service, member::MemberService};
use crate::error::ApiError;

type MemberProvider = State<MemberService<MemberRepository, UserRepository, GroupRepository>>;

pub async fn get_members(
    State(service): MemberProvider
) -> Result<Json<Vec<MemberView>>, ApiError> {
    let members = service.get_all().await?;

    Ok(Json(members))
}

pub async fn get_member(
    State(service): MemberProvider,
    Path(id): Path<Id>
) -> Result<Json<MemberView>, ApiError> {
    let record_id = RecordId::try_from(id)?;

    let member = service
        .get_by_id(record_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(member))
}

pub async fn create_member(
    State(service): MemberProvider,
    Json(draft): Json<MemberDraft>
) -> Result<Json<MemberView>, ApiError> {
    let member = service.create(draft).await?;

    Ok(Json(member))
}
