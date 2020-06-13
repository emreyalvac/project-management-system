use crate::queries::board_queries::get_boards_as_aggregate_query::GetBoardAsAggregateQuery;
use crate::query_handlers::board_query_handlers::get_board_as_aggregate_query_handler::GetBoardAsAggregateQueryHandler;
use async_trait::async_trait;

#[async_trait]
pub trait TBoardQueryHandlerFactory {
    async fn build_for_aggregate(&self, query: GetBoardAsAggregateQuery) -> GetBoardAsAggregateQueryHandler;
}

pub struct BoardQueryHandlerFactory {}

#[async_trait]
impl TBoardQueryHandlerFactory for BoardQueryHandlerFactory {
    async fn build_for_aggregate(&self, query: GetBoardAsAggregateQuery) -> GetBoardAsAggregateQueryHandler {
        GetBoardAsAggregateQueryHandler { query }
    }
}
