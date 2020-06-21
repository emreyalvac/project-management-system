use serde::{Serialize, Deserialize};
use crate::board::board::Board;
use crate::user::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardUsersAggregate {
    pub board: Board,
    pub users: Vec<User>,
}
