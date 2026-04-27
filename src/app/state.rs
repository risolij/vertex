use axum::extract::FromRef;
use crate::service::{GroupService, TaskService, UserService, MemberService};
use crate::repository::{
    user_repository::UserRepository,
    group_repository::GroupRepository,
    task_repository::TaskRepository,
    member_repository::MemberRepository
};

#[derive(Clone)]
pub struct ApplicationState {
    pub tasks: TaskService<TaskRepository, UserRepository, GroupRepository>,
    pub users: UserService<UserRepository>,
    pub groups: GroupService<GroupRepository>,
    pub members: MemberService<MemberRepository>
}

impl ApplicationState {
    pub fn new(
        tasks: TaskService<TaskRepository, UserRepository, GroupRepository>,
        users: UserService<UserRepository>,
        groups: GroupService<GroupRepository>,
        members: MemberService<MemberRepository>,
    ) -> Self {
        Self { tasks, users, groups, members }
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

impl FromRef<ApplicationState> for MemberService<MemberRepository> {
    fn from_ref(state: &ApplicationState) -> Self {
        state.members.clone()
    }
}
