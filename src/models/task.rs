use surrealdb::types::{RecordId, SurrealValue};
use serde::{Deserialize, Serialize};

use super::criticality::{State, Impact, Urgency, Priority};
use crate::service::TaskService;
use crate::repository::task_repository::TaskRepository;
use crate::repository::user_repository::UserRepository;
use crate::repository::Repository;

#[derive(Deserialize, Serialize, SurrealValue)]
pub struct Task {
    id: RecordId,
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
    assigned_to: Option<RecordId>
}

impl TaskService<TaskRepository, UserRepository> {
    pub async fn get_tasks(&self) -> Vec<Task> {
        self.task_repository.list().await
    }

    pub async fn get_task(&self, id: RecordId) -> Option<Task> {
        self.task_repository.get(id).await
    }

    pub async fn create_task(&self, draft: TaskDraft) -> Task {
        let task = if draft.assigned_to.is_some() {
            let user = self.user_repository.get(draft.assigned_to.clone().unwrap()).await;
            self.task_repository.create(draft).await

        } else {
            self.task_repository.create(draft).await
        };

        task
    }
}
