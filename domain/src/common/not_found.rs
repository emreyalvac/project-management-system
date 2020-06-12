use serde::{Serialize, Deserialize};
use crate::common::found_type::FoundType;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotFound {
    pub message: String,
    pub found_type: FoundType,
}
