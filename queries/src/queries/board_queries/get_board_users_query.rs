use domain::query::query::TQuery;
use domain::board::board_users_aggregate::BoardUsersAggregate;

pub struct GetBoardUsersQuery {
    pub board_id: String
}

impl TQuery<BoardUsersAggregate> for GetBoardUsersQuery {}
