use surrealdb::types::{RecordId, SurrealValue, Datetime};
use serde::{Deserialize, Serialize};

use super::criticality::{State, Impact, Urgency, Priority};
use crate::error::ApiError;
use crate::models::id::Id;
use crate::service::{Service, TaskService};
use crate::repository::task_repository::TaskRepo;
use crate::repository::user_repository::UserRepo;
use crate::repository::group_repository::GroupRepo;

#[derive(Deserialize, Serialize, SurrealValue)]
pub struct Task {
    id: Option<RecordId>,
    state: State,
    impact: Impact,
    urgency: Urgency,
    priority: Priority,
    opened: Datetime,
    closed: Option<Datetime>,
    assigned_to: Option<RecordId>
}

struct TaskBuilder {
    state: State,
    impact: Impact,
    urgency: Urgency,
    priority: Priority,
    opened: Datetime,
    assigned_to: Option<RecordId>
}

impl TaskBuilder {
    fn new(state: State, impact: Impact, urgency: Urgency) -> Self {
        let priority = Priority::from(impact.clone(), urgency.clone());

        Self {
            state,
            impact,
            urgency,
            priority,
            opened: Datetime::now(),
            assigned_to: None
        }
    }

    fn assigned_to(mut self, user: Option<RecordId>) -> Self {
        self.assigned_to = user;
        self
    }

    fn build(self) -> Task {
        Task {
            id: None,
            state: self.state,
            impact: self.impact,
            urgency: self.urgency,
            priority: self.priority,
            opened: self.opened,
            closed: None,
            assigned_to: self.assigned_to
        }
    }
}

#[derive(Serialize, Deserialize, SurrealValue)]
pub struct TaskDraft {
    state: Option<State>,
    impact: Option<Impact>,
    urgency: Option<Urgency>,
    assigned_to: Option<String>
}

#[derive(Serialize)]
pub struct TaskView {
    id: RecordId,
    state: State,
    impact: Impact,
    urgency: Urgency,
    priority: Priority,
    opened: Datetime,
    closed: Option<Datetime>
}

impl TryFrom<Task> for TaskView {
    type Error = ApiError;

    fn try_from(task: Task) -> Result<Self, Self::Error> {
        let task = Self {
            id: task.id.ok_or(ApiError::UnprocessableId)?,
            state: task.state,
            impact: task.impact,
            urgency: task.urgency,
            priority: task.priority,
            opened: task.opened,
            closed: task.closed
        };

        Ok(task)
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
