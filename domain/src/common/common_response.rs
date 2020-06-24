use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CommonResponse {
    pub status: bool,
    pub message: String
}
