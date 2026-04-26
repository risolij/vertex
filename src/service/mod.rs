use crate::repository::Repository;
use surrealdb::types::RecordId;

pub trait Service {
    type View;
    type Draft;

    async fn get_by_id(&self, id: RecordId) -> Option<Self::View>;
    async fn get_all(&self) -> Vec<Self::View>;
    async fn create(&self, draft: Self::Draft) -> Self::View;
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
