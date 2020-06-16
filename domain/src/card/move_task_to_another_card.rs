use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MoveTaskToAnotherCard {
    pub from_card_id: String,
    pub to_card_id: String,
    pub task_id: String,
}
