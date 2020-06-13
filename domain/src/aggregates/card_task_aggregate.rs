use serde::{Serialize, Deserialize};
use crate::task::task::Task;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardTaskAggregate {
    #[serde(default)]
    pub card_id: String,
    #[serde(default)]
    pub card_name: String,
    pub tasks: Vec<Task>,
}
