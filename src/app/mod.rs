pub mod state;
pub mod router;

use axum::Router;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use std::sync::Arc;
use crate::service::{GroupService, MemberService, TaskService, UserService};
use crate::repository::{
    task_repository::TaskRepository,
    user_repository::UserRepository,
    member_repository::MemberRepository,
    group_repository::GroupRepository
};

use crate::app::state::ApplicationState;

pub async fn run() {
    let db = Arc::new(Surreal::new::<Mem>(()).await.unwrap());
    db
        .use_ns("app")
        .use_db("app")
        .await
        .unwrap();

    let task_repository = TaskRepository::new(db.clone());
    let user_repository = UserRepository::new(db.clone());
    let group_repository = GroupRepository::new(db.clone());
    let member_repository= MemberRepository::new(db);

    let task_service = TaskService::new(
        task_repository.clone(),
        user_repository.clone(),
        group_repository.clone()
    );

    let group_service = GroupService::new(group_repository);
    let user_service = UserService::new(user_repository);
    let member_service = MemberService::new(member_repository);

    let app_state = ApplicationState::new(
        task_service,
        user_service,
        group_service,
        member_service
    );

    let api_routes = router::create_router(app_state);

    let app = Router::new()
        .nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
