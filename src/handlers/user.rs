use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::{RecordId, record_id};
use crate::repository::user_repository::UserRepository;
use crate::models::user::{User, UserDraft};
use crate::service::UserService;
use crate::error::ApiError;

type UserProvider = State<UserService<UserRepository>>;

pub async fn get_users(
    State(service): UserProvider
) -> Result<Json<Vec<User>>, ApiError> {
    let users = service.get_users().await;

    Ok(Json(users))
}

pub async fn get_user(
    State(service): UserProvider,
    Path(id): Path<String>
) -> Result<Json<User>, ApiError> {
    let record_id = RecordId::parse_simple(&id).map_err(|_| ApiError::NotFound)?;
    let user = service
        .get_user(record_id)
        .await
        .ok_or(ApiError::NotFound)?;

    Ok(Json(user))
}

pub async fn create_user(
    State(service): UserProvider,
    Json(draft): Json<UserDraft>
) -> Result<Json<User>, ApiError> {
    let user = service.create_user(draft).await;

    Ok(Json(user))
}
