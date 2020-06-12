use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub card_id: String,
    pub card_name: String,
    pub card_tasks: Vec<String>,
}
