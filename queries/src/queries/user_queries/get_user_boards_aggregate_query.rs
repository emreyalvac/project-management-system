use domain::query::query::TQuery;
use domain::board::board::Board;

pub struct GetUserBoardsAggregateQuery {
    pub user_id: String
}

impl TQuery<Vec<Board>> for GetUserBoardsAggregateQuery {}
