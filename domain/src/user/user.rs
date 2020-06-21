use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(default, skip_serializing)]
    pub is_validate: bool,
    #[serde(default)]
    pub user_boards: Vec<String>,
}
