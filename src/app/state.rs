use crate::repository::group_repository::GroupRepository;
use crate::service::{GroupService, TaskService, UserService};
use crate::repository::user_repository::UserRepository;
use crate::repository::task_repository::TaskRepository;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct ApplicationState {
    pub tasks: TaskService<TaskRepository, UserRepository, GroupRepository>,
    pub users: UserService<UserRepository>,
    pub groups: GroupService<GroupRepository>
}

impl ApplicationState {
    pub fn new(
        tasks: TaskService<TaskRepository, UserRepository, GroupRepository>,
        users: UserService<UserRepository>,
        groups: GroupService<GroupRepository>
    ) -> Self {
        Self { tasks, users, groups }
    }
}

impl FromRef<ApplicationState> for TaskService<TaskRepository, UserRepository, GroupRepository> {
    fn from_ref(state: &ApplicationState) -> Self {
        state.tasks.clone()
    }
}

impl FromRef<ApplicationState> for UserService<UserRepository> {
    fn from_ref(state: &ApplicationState) -> Self {
        state.users.clone()
    }
}

impl FromRef<ApplicationState> for GroupService<GroupRepository> {
    fn from_ref(state: &ApplicationState) -> Self {
        state.groups.clone()
    }
}
