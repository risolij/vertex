use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[derive(Deserialize, Serialize, SurrealValue)]
pub enum State {
    New,
    InProgress,
    Completed
}

#[derive(Deserialize, Serialize, SurrealValue)]
pub enum Impact {
    Enterprise,
    Department,
    MultipleUsers,
    User
}

#[derive(Deserialize, Serialize, SurrealValue)]
pub enum Urgency {
    High,
    Medium,
    Low
}

#[derive(Deserialize, Serialize, SurrealValue)]
pub enum Priority {
    Critical,
    High,
    Moderate,
    Low
}

impl Priority {
    pub fn from(impact: Impact, urgency: Urgency) -> Self {
        match (impact, urgency) {
            (Impact::Enterprise, Urgency::High) => Priority::Critical,
            (Impact::Enterprise, Urgency::Medium) => Priority::High,
            (Impact::Enterprise, Urgency::Low) => Priority::Moderate,
            (Impact::Department, Urgency::High) => Priority::High,
            (Impact::Department, Urgency::Medium) => Priority::High,
            (Impact::Department, Urgency::Low) => Priority::Moderate,
            (Impact::MultipleUsers, Urgency::High) => Priority::High,
            (Impact::MultipleUsers, Urgency::Medium) => Priority::Moderate,
            (Impact::MultipleUsers, Urgency::Low) => Priority::Moderate,
            (Impact::User, Urgency::High) => Priority::Moderate,
            (Impact::User, Urgency::Medium) => Priority::Low,
            (Impact::User, Urgency::Low) => Priority::Low,
        }
    }
}
