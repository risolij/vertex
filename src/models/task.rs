use surrealdb::types::{RecordId, SurrealValue};
use serde::{Deserialize, Serialize};

use super::criticality::{State, Impact, Urgency, Priority};
use crate::service::TaskService;
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
        Task {
            id: None,
            state: draft.state,
            impact: draft.impact,
            urgency: draft.urgency,
            priority: draft.priority,
            assigned_to: draft.assigned_to.map(|id| RecordId::parse_simple(&id).unwrap())
        }
    }
}

impl TaskService<TaskRepository, UserRepository> {
    pub async fn get_tasks(&self) -> Vec<Task> {
        self.task_repository.list().await
    }

    pub async fn get_task(&self, id: RecordId) -> Option<Task> {
        self.task_repository.get(id).await
    }

    pub async fn create_task(&self, draft: TaskDraft) -> Task {
        let task = Task::from(draft);
        let task = self.task_repository.create(task).await;

        task
    }
}
