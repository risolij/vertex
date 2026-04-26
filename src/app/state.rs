use crate::service::{TaskService, UserService};
use crate::repository::user_repository::UserRepository;
use crate::repository::task_repository::TaskRepository;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct ApplicationState {
    pub tasks: TaskService<TaskRepository, UserRepository>,
    pub users: UserService<UserRepository>
}

impl ApplicationState {
    pub fn new(tasks: TaskService<TaskRepository, UserRepository>, users: UserService<UserRepository>) -> Self {
        Self { tasks, users }
    }
}

impl FromRef<ApplicationState> for TaskService<TaskRepository, UserRepository> {
    fn from_ref(state: &ApplicationState) -> Self {
        state.tasks.clone()
    }
}

impl FromRef<ApplicationState> for UserService<UserRepository> {
    fn from_ref(state: &ApplicationState) -> Self {
        state.users.clone()
    }
}
