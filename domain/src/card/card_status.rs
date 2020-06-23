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

impl From<CardStatus> for bson::Bson {
    fn from(value: CardStatus) -> Self {
        match value {
            CardStatus::InProgress => bson::Bson::String("InProgress".to_owned()),
            CardStatus::Completed => bson::Bson::String("Completed".to_owned()),
        }
    }
}
