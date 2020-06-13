use domain::query::query::TQuery;
use domain::aggregates::board_aggregate::BoardAggregate;

pub struct GetBoardAsAggregateQuery {
    pub board_id: String
}

impl TQuery<BoardAggregate> for GetBoardAsAggregateQuery {}
