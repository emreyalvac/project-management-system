use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}
