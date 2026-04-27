use crate::error::ApiError; 
use crate::repository::Repository;

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
    R: Repository
{
    pub repository: R
}

impl<R> UserService<R>
where
    R: Repository
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[derive(Clone)]
pub struct TaskService<T, U, G>
where
    T: Repository,
    U: Repository,
    G: Repository
{
    pub task_repository: T,
    pub user_repository: U,
    pub group_repository: G
}

impl<T, U, G> TaskService<T, U, G>
where
    T: Repository,
    U: Repository,
    G: Repository
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
    G: Repository
{
    pub repository: G
}

impl<G> GroupService<G>
where
    G: Repository
{
    pub fn new(repository: G) -> Self {
        Self {
            repository
        }
    }
}

#[derive(Clone)]
pub struct MemberService<M>
where
    M: Repository
{
    pub repository: M

}

impl<M> MemberService<M>
where
    M: Repository
{
    pub fn new(repository: M) -> Self {
        Self {
            repository
        }
    }
}
