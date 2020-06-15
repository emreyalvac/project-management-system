use serde::{Serialize, Deserialize};
use uuid::Uuid;

fn id_generator() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InsertBoardToUser {
    #[serde(default)]
    pub user_id: String,
    #[serde(default = "id_generator")]
    pub board_id: String,
    pub board_name: String,
    #[serde(default)]
    pub board_manager_user_id: String,
    #[serde(default)]
    pub board_cards: Vec<String>,
}
