use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::RecordId;
use crate::repository::user_repository::UserRepository;
use crate::models::user::{UserView, UserDraft};
use crate::service::{Service, UserService};
use crate::error::ApiError;

type UserProvider = State<UserService<UserRepository>>;

pub async fn get_users(
    State(service): UserProvider
) -> Result<Json<Vec<UserView>>, ApiError> {
    let users = service.get_all().await;

    Ok(Json(users))
}

pub async fn get_user(
    State(service): UserProvider,
    Path(id): Path<String>
) -> Result<Json<UserView>, ApiError> {
    let record_id = RecordId::parse_simple(&id).map_err(|_| ApiError::NotFound)?;
    let user = service
        .get_by_id(record_id)
        .await
        .ok_or(ApiError::NotFound)?;

    Ok(Json(user))
}

pub async fn create_user(
    State(service): UserProvider,
    Json(draft): Json<UserDraft>
) -> Result<Json<UserView>, ApiError> {
    let user = service.create(draft).await;

    Ok(Json(user))
}
