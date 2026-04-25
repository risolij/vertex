use axum::Router;
use axum::routing::{get, post};

use crate::handlers::{health, task, user};
use super::state::ApplicationState;

pub fn create_router(state: ApplicationState) -> Router {
    Router::new()
        .route("/ruok", get(health::ruok))
        .nest("/users", user_routes())
        .nest("/tasks", task_routes())
        .with_state(state)
}

fn user_routes() -> Router<ApplicationState> {
    Router::new()
        .route("/", get(user::get_users).post(user::create_user))
        .route("/{id}", get(user::get_user))
}

fn task_routes() -> Router<ApplicationState> {
        Router::new()
        .route("/", get(task::get_tasks).post(task::create_task))
        .route("/{id}", get(task::get_task))
}
