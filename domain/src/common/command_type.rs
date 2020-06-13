use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandType {
    BoardInsert,
    UserInsert,
    ValidateUser,
}
