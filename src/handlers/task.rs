use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::RecordId;
use crate::models::id::Id;
use crate::repository::group_repository::GroupRepository;
use crate::repository::user_repository::UserRepository;
use crate::repository::task_repository::TaskRepository;
use crate::service::{Service, TaskService};
use crate::models::task::{TaskDraft, TaskView};
use crate::error::ApiError;

type TaskProvider = State<TaskService<TaskRepository, UserRepository, GroupRepository>>;

pub async fn get_tasks(
    State(service): TaskProvider
) -> Result<Json<Vec<TaskView>>, ApiError> {
    let tasks = service.get_all().await?;

    Ok(Json(tasks))
}

pub async fn get_task(
    State(service): TaskProvider,
    Path(id): Path<Id>
) -> Result<Json<TaskView>, ApiError> {
    let record_id = RecordId::try_from(id)?;

    let task = service
        .get_by_id(record_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(task))
}

pub async fn create_task(
    State(service): TaskProvider,
    Json(draft): Json<TaskDraft>
) -> Result<Json<TaskView>, ApiError> {
    let task = service.create(draft).await?;

    Ok(Json(task))
}
