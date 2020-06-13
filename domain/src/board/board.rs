use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub board_id: String,
    pub board_name: String,
    pub board_manager_user_id: String,
}