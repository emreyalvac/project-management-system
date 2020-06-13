use crate::query_handlers::user_query_handlers::user_get_by_email_query_handler::UserGetByEmailQueryHandler;
use crate::queries::user_queries::user_get_by_email_query::UserGetByEmailQuery;
use async_trait::async_trait;
use crate::queries::user_queries::get_user_boards_aggregate_query::GetUserBoardsAggregateQuery;
use crate::query_handlers::user_query_handlers::get_user_boards_aggregate_query_handler::GetUserBoardsAggregateQueryHandler;

#[async_trait]
pub trait TUserQueryHandlerFactory {
    async fn build_for_email(&self, query: UserGetByEmailQuery) -> UserGetByEmailQueryHandler;
    async fn build_for_boards(&self, query: GetUserBoardsAggregateQuery) -> GetUserBoardsAggregateQueryHandler;
}

pub struct UserQueryHandlerFactory {}

#[async_trait]
impl TUserQueryHandlerFactory for UserQueryHandlerFactory {
    async fn build_for_email(&self, query: UserGetByEmailQuery) -> UserGetByEmailQueryHandler {
        UserGetByEmailQueryHandler { query }
    }

    async fn build_for_boards(&self, query: GetUserBoardsAggregateQuery) -> GetUserBoardsAggregateQueryHandler {
        GetUserBoardsAggregateQueryHandler { query }
    }
}
