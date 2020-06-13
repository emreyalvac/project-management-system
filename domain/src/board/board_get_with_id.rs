use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BoardGetWithId {
    pub board_id: String
}
