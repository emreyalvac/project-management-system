use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteTask {
    pub task_id: String
}
