use serde::{Serialize, Deserialize};
use crate::task::task_status::TaskStatus;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateTask {
    pub task_id: String,
    pub task_name: String,
    pub task_start_date: String,
    pub task_end_date: String,
    pub task_description: String,
    pub task_status: TaskStatus,
}
