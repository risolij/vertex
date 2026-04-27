use surrealdb::types::{RecordId, SurrealValue, Datetime};
use serde::{Deserialize, Serialize};

use super::criticality::{State, Impact, Urgency, Priority};
use crate::error::ApiError;

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

pub struct TaskBuilder {
    state: State,
    impact: Impact,
    urgency: Urgency,
    priority: Priority,
    opened: Datetime,
    assigned_to: Option<RecordId>
}

impl TaskBuilder {
    pub fn new(state: State, impact: Impact, urgency: Urgency) -> Self {
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

    pub fn assigned_to(mut self, user: Option<RecordId>) -> Self {
        self.assigned_to = user;
        self
    }

    pub fn build(self) -> Task {
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
    pub state: Option<State>,
    pub impact: Option<Impact>,
    pub urgency: Option<Urgency>,
    pub assigned_to: Option<String>
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
