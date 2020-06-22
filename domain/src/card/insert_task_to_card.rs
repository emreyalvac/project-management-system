use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::task::task_status::TaskStatus;

fn id_generator() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InsertTask {
    #[serde(skip_serializing)]
    pub card_id: String,
    #[serde(default = "id_generator")]
    pub task_id: String,
    pub task_name: String,
    pub task_start_date: String,
    pub task_end_date: String,
    #[serde(default)]
    pub task_assigned_users: Vec<String>,
    pub task_description: String,
    #[serde(default = "TaskStatus::default_status")]
    pub task_status: TaskStatus,
}
