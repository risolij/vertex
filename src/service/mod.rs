use crate::error::ApiError; 
use crate::repository::group_repository::GroupRepo;
use crate::repository::member_repository::MemberRepo;
use crate::repository::task_repository::TaskRepo;
use crate::repository::user_repository::UserRepo;

pub trait Service {
    type View;
    type Draft;
    type Id;

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError>;
    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError>;
    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError>;
}

#[derive(Clone)]
pub struct UserService<R>
where
    R: UserRepo
{
    pub repository: R
}

impl<R> UserService<R>
where
    R: UserRepo
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[derive(Clone)]
pub struct TaskService<T, U, G>
where
    T: TaskRepo,
    U: UserRepo,
    G: GroupRepo
{
    pub task_repository: T,
    pub user_repository: U,
    pub group_repository: G
}

impl<T, U, G> TaskService<T, U, G>
where
    T: TaskRepo,
    U: UserRepo,
    G: GroupRepo
{
    pub fn new(task_repository: T, user_repository: U, group_repository: G) -> Self {
        Self {
            task_repository,
            user_repository,
            group_repository
        }
    }
}

#[derive(Clone)]
pub struct GroupService<G>
where
    G: GroupRepo
{
    pub repository: G
}

impl<G> GroupService<G>
where
    G: GroupRepo
{
    pub fn new(repository: G) -> Self {
        Self {
            repository
        }
    }
}

#[derive(Clone)]
pub struct MemberService<M, U, G>
where
    M: MemberRepo,
    U: UserRepo,
    G: GroupRepo
{
    pub member_repository: M,
    pub user_repository: U,
    pub group_repository: G

}

impl<M, U, G> MemberService<M, U, G>
where
    M: MemberRepo,
    U: UserRepo,
    G: GroupRepo
{
    pub fn new(member_repository: M, user_repository: U, group_repository: G) -> Self {
        Self {
            member_repository,
            user_repository,
            group_repository
        }
    }
}
