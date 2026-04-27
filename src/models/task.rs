use surrealdb::types::{RecordId, SurrealValue, Datetime};
use serde::{Deserialize, Serialize};

use super::criticality::{State, Impact, Urgency, Priority};
use crate::error::ApiError;
use crate::repository::group_repository::GroupRepository;
use crate::service::{Service, TaskService};
use crate::repository::task_repository::TaskRepository;
use crate::repository::user_repository::UserRepository;
use crate::repository::Repository;

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

#[derive(Serialize, Deserialize, SurrealValue)]
pub struct TaskDraft {
    state: Option<State>,
    impact: Option<Impact>,
    urgency: Option<Urgency>,
    assigned_to: Option<String>
}

impl TryFrom<TaskDraft> for Task {
    type Error = ApiError;

    fn try_from(draft: TaskDraft) -> Result<Self, Self::Error> {
        let user_id = draft
            .assigned_to
            .map(|id| {
                RecordId::parse_simple(&id)
            })
            .ok_or(ApiError::InternalServerError)?;

        let state = draft.state.unwrap_or_default();
        let impact = draft.impact.unwrap_or_default();
        let urgency = draft.urgency.unwrap_or_default();
        let priority = Priority::from(impact.clone(), urgency.clone());

        let task = Self {
            id: None,
            state,
            impact,
            urgency,
            priority,
            opened: Datetime::now(),
            closed: None,
            assigned_to: user_id.ok()
        };

        Ok(task)
    }
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

impl From<Task> for TaskView {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.unwrap(),
            state: task.state,
            impact: task.impact,
            urgency: task.urgency,
            priority: task.priority,
            opened: task.opened,
            closed: task.closed
        }
    }
}

impl Service for TaskService<TaskRepository, UserRepository, GroupRepository> {
    type View = TaskView;
    type Draft = TaskDraft;
    type Id = RecordId;

    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::View>, ApiError> {
        let view = self.task_repository
            .get(id)
            .await?
            .map(|task| TaskView::from(task));

        Ok(view)
    }

    async fn get_all(&self) -> Result<Vec<Self::View>, ApiError> {
        let views = self.task_repository
            .list()
            .await?
            .into_iter()
            .map(|task| TaskView::from(task))
            .collect();

        Ok(views)
        
    }

    async fn create(&self, draft: Self::Draft) -> Result<Self::View, ApiError> {
        let task = Task::try_from(draft)?;
        let task = self.task_repository.create(task).await?;

        Ok(TaskView::from(task))
    }
}
