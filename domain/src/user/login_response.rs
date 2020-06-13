use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub status: bool,
    pub token: String,
}
