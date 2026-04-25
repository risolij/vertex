use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::RecordId;
use crate::repository::user_repository::UserRepository;
use crate::repository::task_repository::TaskRepository;
use crate::service::TaskService;
use crate::models::task::{Task, TaskDraft};

type TaskProvider = State<TaskService<TaskRepository, UserRepository>>;

pub async fn get_tasks(
    State(service): TaskProvider
) -> Json<Vec<Task>> {
    Json(service.get_tasks().await)
}

pub async fn get_task(
    State(service): TaskProvider,
    Path(id): Path<RecordId>
) -> Json<Task> {
    Json(service.get_task(id).await.unwrap())
}

pub async fn create_task(
    State(service): TaskProvider,
    Json(draft): Json<TaskDraft>
) -> Json<Task> {
    Json(service.create_task(draft).await)
}
