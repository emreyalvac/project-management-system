use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CardStatus {
    Completed,
    InProgress,
}

impl Default for CardStatus {
    fn default() -> Self {
        CardStatus::InProgress
    }
}
