use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserGetById {
    pub user_id: String
}
