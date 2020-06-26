use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateUser {
    #[serde(skip_serializing, default)]
    pub user_id: String,
    pub name: String,
    pub surname: String,
    pub user_name: String,
}
