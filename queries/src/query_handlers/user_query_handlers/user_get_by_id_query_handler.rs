use crate::queries::user_queries::user_get_by_id_query::UserGetByIdQuery;
use domain::query::query::TQueryHandler;
use domain::user::user::User;
use domain::common::not_found::NotFound;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use async_trait::async_trait;
use domain::common::found_type::FoundType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};

pub struct UserGetByIdQueryHandler {
    pub query: UserGetByIdQuery
}

#[async_trait]
impl TQueryHandler<UserGetByIdQuery, User, NotFound> for UserGetByIdQueryHandler {
    async fn get(&self) -> Result<User, NotFound> {
        let database = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository = GenericRepository { collection: "users".to_owned(), connection: client };
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
            Err(_) => {
                Err(NotFound { found_type: FoundType::User, message: "Not found".to_owned() })
            }
        }
    }
}
