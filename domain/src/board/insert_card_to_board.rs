use serde::{Serialize, Deserialize};
use uuid::Uuid;

fn id_generator() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InsertCardToBoard {
    #[serde(default = "id_generator")]
    pub card_id: String,
    #[serde(skip_serializing)]
    pub board_id: String,
    pub card_name: String,
    #[serde(default)]
    pub card_tasks: Vec<String>,
}
