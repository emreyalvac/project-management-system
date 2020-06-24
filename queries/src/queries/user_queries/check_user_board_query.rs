use domain::query::query::TQuery;

pub struct CheckUserBoardQuery {
    pub board_id: String,
    pub user_id: String
}

impl TQuery<bool> for CheckUserBoardQuery {}
