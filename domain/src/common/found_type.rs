use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FoundType {
    News,
    Category,
    User,
}
