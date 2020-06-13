use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub task_id: String,
    pub task_name: String,
    pub task_expire_date: String,
    pub task_assigned_user_id: String,
    pub task_description: String,
}
