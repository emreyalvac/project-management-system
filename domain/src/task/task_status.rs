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

impl From<TaskStatus> for bson::Bson {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::InProgress => bson::Bson::String("InProgress".to_owned()),
            TaskStatus::Completed => bson::Bson::String("Completed".to_owned())
        }
    }
}
