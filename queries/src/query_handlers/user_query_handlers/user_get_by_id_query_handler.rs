use crate::queries::user_queries::user_get_by_id_query::UserGetByIdQuery;
use domain::query::query::TQueryHandler;
use domain::user::user::User;
use domain::common::not_found::NotFound;
use async_trait::async_trait;
use domain::common::found_type::FoundType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::Client;

pub struct UserGetByIdQueryHandler {
    pub query: UserGetByIdQuery,
    pub client: Client,
}

#[async_trait]
impl TQueryHandler<UserGetByIdQuery, User, NotFound> for UserGetByIdQueryHandler {
    async fn get(&self) -> Result<User, NotFound> {
        let connection = self.client.to_owned();
        let repository = GenericRepository { collection: "users".to_owned(), connection };
        let user_id = &self.query.id;
        let handler = repository.get_by_generic::<User>("id".to_owned(), (user_id).parse().unwrap()).await;
        match handler {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                Err(NotFound { found_type: FoundType::User, message: "Not found".to_owned() })
            }
        }
    }
}
