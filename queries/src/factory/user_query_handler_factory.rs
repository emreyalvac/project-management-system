use crate::query_handlers::user_query_handlers::user_get_by_email_query_handler::UserGetByEmailQueryHandler;
use crate::queries::user_queries::user_get_by_email_query::UserGetByEmailQuery;
use async_trait::async_trait;
use crate::queries::user_queries::get_user_boards_aggregate_query::GetUserBoardsAggregateQuery;
use crate::query_handlers::user_query_handlers::get_user_boards_aggregate_query_handler::GetUserBoardsAggregateQueryHandler;
use crate::queries::user_queries::user_get_by_id_query::UserGetByIdQuery;
use crate::query_handlers::user_query_handlers::user_get_by_id_query_handler::UserGetByIdQueryHandler;
use crate::queries::user_queries::check_user_board_query::CheckUserBoardQuery;
use crate::query_handlers::user_query_handlers::check_user_board_query_handler::CheckUserBoardQueryHandler;
use mongodb::Client;

#[async_trait]
pub trait TUserQueryHandlerFactory {
    async fn build_for_email(&self, query: UserGetByEmailQuery) -> UserGetByEmailQueryHandler;
    async fn build_for_boards(&self, query: GetUserBoardsAggregateQuery) -> GetUserBoardsAggregateQueryHandler;
    async fn build_for_get_by_id(&self, query: UserGetByIdQuery) -> UserGetByIdQueryHandler;
    async fn build_for_check_user_board(&self, query: CheckUserBoardQuery) -> CheckUserBoardQueryHandler;
}

pub struct UserQueryHandlerFactory {
    pub client: Client
}

#[async_trait]
impl TUserQueryHandlerFactory for UserQueryHandlerFactory {
    async fn build_for_email(&self, query: UserGetByEmailQuery) -> UserGetByEmailQueryHandler {
        UserGetByEmailQueryHandler { query, client: self.client.to_owned() }
    }

    async fn build_for_boards(&self, query: GetUserBoardsAggregateQuery) -> GetUserBoardsAggregateQueryHandler {
        GetUserBoardsAggregateQueryHandler { query, client: self.client.to_owned() }
    }

    async fn build_for_get_by_id(&self, query: UserGetByIdQuery) -> UserGetByIdQueryHandler {
        UserGetByIdQueryHandler { query, client: self.client.to_owned() }
    }

    async fn build_for_check_user_board(&self, query: CheckUserBoardQuery) -> CheckUserBoardQueryHandler {
        CheckUserBoardQueryHandler { query, client: self.client.to_owned() }
    }
}
