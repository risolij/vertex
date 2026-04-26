pub mod state;
pub mod router;

use axum::Router;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use std::sync::Arc;
use crate::service::{UserService, TaskService};
use crate::repository::Repository;
use crate::repository::task_repository::TaskRepository;
use crate::repository::user_repository::UserRepository;

use crate::app::state::ApplicationState;

pub async fn run() {
    let db = Arc::new(Surreal::new::<Mem>(()).await.unwrap());
    db
        .use_ns("app")
        .use_db("app")
        .await
        .unwrap();

    let task_repository = TaskRepository::new(db.clone());
    let user_repository = UserRepository::new(db);

    let task_service = TaskService::new(
        task_repository.clone(),
        user_repository.clone()
    );
    let user_service = UserService::new(user_repository);

    let app_state = ApplicationState::new(
        task_service,
        user_service
    );

    let api_routes = router::create_router(app_state);

    let app = Router::new()
        .nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
