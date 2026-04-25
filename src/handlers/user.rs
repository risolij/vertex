use axum::extract::{Path, State};
use axum::Json;
use surrealdb::types::RecordId;
use crate::repository::user_repository::UserRepository;
use crate::models::user::{User, UserDraft};
use crate::service::Service;

type UserProvider = State<Service<UserRepository>>;

pub async fn get_users(
    State(service): UserProvider
) -> Json<Vec<User>> {
    Json(service.get_users().await)
}

pub async fn get_user(
    State(service): UserProvider,
    Path(id): Path<RecordId>
) -> Json<User> {
    Json(service.get_user(id).await.unwrap())
}

pub async fn create_user(
    State(service): UserProvider,
    Json(draft): Json<UserDraft>
) -> Json<User> {
    Json(service.create_user(draft).await)
}
