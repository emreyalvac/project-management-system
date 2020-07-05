use serde::{Serialize, Deserialize};
use crate::board::board::Board;
use crate::aggregates::card_task_aggregate::CardTaskAggregate;

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardAggregate {
    pub board: Board,
    pub cards: Vec<CardTaskAggregate>,
}
