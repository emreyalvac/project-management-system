use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetByEmail {
    pub email: String
}
