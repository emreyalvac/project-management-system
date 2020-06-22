use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::board::board_status::BoardStatus;

fn id_generator() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Serialize, Deserialize)]
pub struct InsertableBoard {
    #[serde(default = "id_generator")]
    pub board_id: String,
    pub board_name: String,
    pub board_manager_user_id: String,
    #[serde(default)]
    pub board_cards: Vec<String>,
    #[serde(default)]
    pub board_status: BoardStatus,
}
