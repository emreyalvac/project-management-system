use serde::{Serialize, Deserialize};
use crate::card::card_status::CardStatus;

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub card_id: String,
    pub card_name: String,
    pub card_tasks: Vec<String>,
    pub card_status: CardStatus,
}
