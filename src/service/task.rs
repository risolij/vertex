use super::Service;
use crate::error::ApiError;
use crate::models::task::{TaskBuilder, TaskView, TaskDraft};
use crate::models::id::Id;
use crate::repository::{
    group_repository::GroupRepo,
    user_repository::UserRepo,
    task_repository::TaskRepo
};
use surrealdb::types::RecordId;

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

impl<T, U, G> Service for TaskService<T, U, G>
where
    T: TaskRepo,
    U: UserRepo,
    G: GroupRepo
{
    type View = TaskView;
    type Draft = TaskDraft;
    type Id = RecordId;

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError> {
        self.task_repository
            .get(id)
            .await?
            .map(TaskView::try_from)
            .transpose()
    }

    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError> {
        self.task_repository
            .list()
            .await?
            .into_iter()
            .map(TaskView::try_from)
            .collect()
    }

    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError> {
        let state = draft.state.unwrap_or_default();
        let impact = draft.impact.unwrap_or_default();
        let urgency = draft.urgency.unwrap_or_default();

        let assigned_to = match draft.assigned_to {
            Some(id) => {
                let id = Id(id);
                let id = RecordId::try_from(id)?;

                let exists = self.user_repository
                    .get(id.clone())
                    .await?
                    .is_some();

                exists.then_some(id)

            },
            None => None,
        };

        let task = TaskBuilder::new(state, impact, urgency)
            .assigned_to(assigned_to)
            .build();

        let saved= self.task_repository.create(task).await?;

        TaskView::try_from(saved)
    }
}
