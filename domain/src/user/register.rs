use serde::{Serialize, Deserialize};
use uuid::Uuid;

fn id_generator() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Register {
    #[serde(default = "id_generator")]
    pub id: String,
    pub email: String,
    pub user_name: String,
    pub password: String,
    #[serde(default)]
    pub is_validate: bool,
    #[serde(default)]
    pub user_boards: Vec<String>,
}
