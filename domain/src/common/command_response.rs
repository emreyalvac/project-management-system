use serde::{Serialize, Deserialize};
use crate::common::command_type::CommandType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub status: bool,
    pub message: String,
    pub command_type: CommandType,
}
