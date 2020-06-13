use serde::{Serialize, Deserialize};
use crate::board::board::Board;

// TODO : Check board_id exception

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardUserAggregate {
    pub boards: Vec<Board>
}
