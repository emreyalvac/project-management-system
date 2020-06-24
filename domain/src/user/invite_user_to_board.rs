use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InviteUserToBoard {
    pub board_id: String,
    pub email: String,
}
