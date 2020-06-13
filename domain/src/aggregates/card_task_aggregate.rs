use serde::{Serialize, Deserialize};
use crate::task::task::Task;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardTaskAggregate {
    pub card_id: String,
    pub card_name: String,
    pub tasks: Vec<Task>,
}
