use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::RecordId;
use crate::models::group::{GroupDraft, GroupView};
use crate::repository::group_repository::GroupRepository;
use crate::service::{GroupService, Service};
use crate::error::ApiError;

type GroupProvider = State<GroupService<GroupRepository>>;

pub async fn get_groups(
    State(service): GroupProvider,
) -> Result<Json<Vec<GroupView>>, ApiError> {
    let groups = service.get_all().await?;

    Ok(Json(groups))
}

pub async fn get_group(
    State(service): GroupProvider,
    Path(id): Path<String>
) -> Result<Json<GroupView>, ApiError> {
    let record_id = RecordId::parse_simple(&id).map_err(|_| ApiError::NotFound)?;
    let group = service
        .get_by_id(record_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(group))
}

pub async fn create_group(
    State(service): GroupProvider,
    Json(draft): Json<GroupDraft>
) -> Result<Json<GroupView>, ApiError> {
    let group = service.create(draft).await?;

    Ok(Json(group))
}
