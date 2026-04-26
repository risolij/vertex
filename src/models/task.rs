use surrealdb::types::{RecordId, SurrealValue};
use serde::{Deserialize, Serialize};

use super::criticality::{State, Impact, Urgency, Priority};
use crate::error::ApiError;
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
    assigned_to: Option<RecordId>
}

#[derive(Serialize, Deserialize, SurrealValue)]
pub struct TaskDraft {
    state: State,
    impact: Impact,
    urgency: Urgency,
    priority: Priority,
    assigned_to: Option<String>
}

impl From<TaskDraft> for Task {
    fn from(draft: TaskDraft) -> Self {
        Self {
            id: None,
            state: draft.state,
            impact: draft.impact,
            urgency: draft.urgency,
            priority: draft.priority,
            assigned_to: draft.assigned_to.map(|id| RecordId::parse_simple(&id).unwrap())
        }
    }
}

#[derive(Serialize, SurrealValue)]
pub struct TaskView {
    id: RecordId,
    state: State,
    impact: Impact,
    urgency: Urgency,
    priority: Priority
}

impl From<Task> for TaskView {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.unwrap(),
            state: task.state,
            impact: task.impact,
            urgency: task.urgency,
            priority: task.priority
        }
    }
}

impl Service for TaskService<TaskRepository, UserRepository> {
    type View = TaskView;
    type Draft = TaskDraft;

    async fn get_by_id(&self, id: RecordId) -> Result<Option<Self::View>, ApiError> {
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
        let task = Task::from(draft);
        let task = self.task_repository.create(task).await?;
        Ok(TaskView::from(task))
    }
}
