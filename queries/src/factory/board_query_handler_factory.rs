use crate::queries::board_queries::get_boards_as_aggregate_query::GetBoardAsAggregateQuery;
use crate::query_handlers::board_query_handlers::get_board_as_aggregate_query_handler::GetBoardAsAggregateQueryHandler;
use async_trait::async_trait;
use crate::queries::board_queries::get_board_users_query::GetBoardUsersQuery;
use crate::query_handlers::board_query_handlers::get_board_users_query_handler::GetBoardUsersQueryHandler;
use mongodb::Client;

#[async_trait]
pub trait TBoardQueryHandlerFactory {
    async fn build_for_aggregate(&self, query: GetBoardAsAggregateQuery) -> GetBoardAsAggregateQueryHandler;
    async fn build_for_get_board_users(&self, query: GetBoardUsersQuery) -> GetBoardUsersQueryHandler;
}

pub struct BoardQueryHandlerFactory {
    pub client: Client
}

#[async_trait]
impl TBoardQueryHandlerFactory for BoardQueryHandlerFactory {
    async fn build_for_aggregate(&self, query: GetBoardAsAggregateQuery) -> GetBoardAsAggregateQueryHandler {
        GetBoardAsAggregateQueryHandler { query, client: self.client.to_owned() }
    }

    async fn build_for_get_board_users(&self, query: GetBoardUsersQuery) -> GetBoardUsersQueryHandler {
        GetBoardUsersQueryHandler { query, client: self.client.to_owned() }
    }
}
