use domain::query::query::TQuery;
use domain::board::board::Board;

pub struct GetAllBoardQuery {}

impl TQuery<Vec<Board>> for GetAllBoardQuery {}
