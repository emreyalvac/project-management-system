use serde::{Serialize, Deserialize};
use crate::task::task_status::TaskStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub task_id: String,
    pub task_name: String,
    pub task_start_date: String,
    pub task_end_date: String,
    pub task_assigned_users: Vec<String>,
    pub task_description: String,
    pub task_status: TaskStatus,
}
