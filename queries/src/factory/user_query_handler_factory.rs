use crate::query_handlers::user_query_handlers::user_get_by_email_query_handler::UserGetByEmailQueryHandler;
use crate::queries::user_queries::user_get_by_email_query::UserGetByEmailQuery;

pub trait TUserQueryHandlerFactory {
    fn build_for_email(&self, query: UserGetByEmailQuery) -> UserGetByEmailQueryHandler;
}

pub struct UserQueryHandlerFactory {}

impl TUserQueryHandlerFactory for UserQueryHandlerFactory {
    fn build_for_email(&self, query: UserGetByEmailQuery) -> UserGetByEmailQueryHandler {
        UserGetByEmailQueryHandler { query }
    }
}
