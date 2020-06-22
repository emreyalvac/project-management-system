use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BoardStatus {
    Completed,
    InProgress,
}

impl Default for BoardStatus {
    fn default() -> Self {
        BoardStatus::InProgress
    }
}
