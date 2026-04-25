use axum::Router;
use axum::routing::get;
use axum::routing::post;

use crate::app::state::ApplicationState;
use user::{get_users, get_user, create_user};
use task::{get_tasks, get_task, create_task};
use health::ruok;

pub mod health;
pub mod task;
pub mod user;

pub fn create_router(state: ApplicationState) -> Router {
    Router::new()
        .route("/ruok", get(ruok))
        .nest("/users", user_routes())
        .nest("/tasks", task_routes())
        .with_state(state)
}

fn user_routes() -> Router<ApplicationState> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/{id}", get(get_user))
}

fn task_routes() -> Router<ApplicationState> {
        Router::new()
        .route("/", get(get_tasks).post(create_task))
        .route("/{id}", get(get_task))

}
