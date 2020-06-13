use serde::{Serialize, Deserialize};
use crate::board::board::Board;

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardUserAggregate {
    pub boards: Vec<Board>
}
