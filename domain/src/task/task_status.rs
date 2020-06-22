use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Completed,
    InProgress,
}

impl TaskStatus {
    pub fn default_status() -> Self {
        TaskStatus::InProgress
    }
}
