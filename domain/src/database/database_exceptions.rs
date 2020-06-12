use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DatabaseException {
    pub message: String
}
