use serde::{Serialize, Deserialize};
use crate::card::card_status::CardStatus;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateCard {
    pub card_id: String,
    pub card_name: String,
    pub card_status: CardStatus,
}
