use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct InviteUserResponse {
    pub email: String,
    pub token: String
}
