use crate::{error::ApiError, repository::Repository};
use surrealdb::types::RecordId;

pub trait Service {
    type View;
    type Draft;

    async fn get_by_id(&self, id: RecordId) -> Result<Option<Self::View>, ApiError>;
    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError>;
    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError>;
}

#[derive(Clone)]
pub struct UserService<R> where R: Repository {
    pub repository: R
}

impl<R> UserService<R> where R: Repository {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[derive(Clone)]
pub struct TaskService<T, U>
where
    T: Repository,
    U: Repository
{
    pub task_repository: T,
    pub user_repository: U
}

impl<T, U> TaskService<T, U>
where
    T: Repository,
    U: Repository
{
    pub fn new(task_repository: T, user_repository: U) -> Self {
        Self {
            task_repository,
            user_repository
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
