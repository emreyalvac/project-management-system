use serde::{Deserialize};

#[derive(Deserialize)]
pub struct DeleteAssignedUserFromTask {
    pub task_id: String,
    pub user_id: String,
}
