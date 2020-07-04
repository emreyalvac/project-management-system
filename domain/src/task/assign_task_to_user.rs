use serde::{Deserialize};

#[derive(Deserialize)]
pub struct AssignTaskToUser {
    pub task_id: String,
    pub user_id: String,
}
