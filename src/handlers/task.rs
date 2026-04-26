use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::RecordId;
use crate::repository::user_repository::UserRepository;
use crate::repository::task_repository::TaskRepository;
use crate::service::TaskService;
use crate::models::task::{Task, TaskDraft};
use crate::error::ApiError;

type TaskProvider = State<TaskService<TaskRepository, UserRepository>>;

pub async fn get_tasks(
    State(service): TaskProvider
) -> Result<Json<Vec<Task>>, ApiError> {
    let tasks = service.get_tasks().await;

    Ok(Json(tasks))
}

pub async fn get_task(
    State(service): TaskProvider,
    Path(id): Path<String>
) -> Result<Json<Task>, ApiError> {
    let record_id = RecordId::parse_simple(&id)
        .map_err(|e| ApiError::NotFound)?;

    let task = service
        .get_task(record_id)
        .await
        .ok_or(ApiError::NotFound)?;

    Ok(Json(task))
}

pub async fn create_task(
    State(service): TaskProvider,
    Json(draft): Json<TaskDraft>
) -> Result<Json<Task>, ApiError> {
    let task = service.create_task(draft).await;

    Ok(Json(task))
}
