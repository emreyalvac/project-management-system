use crate::queries::user_queries::user_get_by_email_query::UserGetByEmailQuery;
use domain::common::not_found::NotFound;
use async_trait::async_trait;
use domain::user::user::User;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use domain::common::found_type::FoundType;
use domain::query::query::TQueryHandler;

pub struct UserGetByEmailQueryHandler {
    pub query: UserGetByEmailQuery
}

#[async_trait]
impl TQueryHandler<UserGetByEmailQuery, User, NotFound> for UserGetByEmailQueryHandler {
    async fn get(&self) -> Result<User, NotFound> {
        let database: DatabaseConnection = DatabaseConnection {};
        let connection = database.get_connection().await.ok().unwrap();
        let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection };
        let result = repository.get_by_generic::<User>("email".to_owned(), (&self.query.email).parse().unwrap()).await;
        match result {
            Ok(user) => {
                Ok(user)
            }
            Err(_) => {
                Err(NotFound { found_type: FoundType::User, message: "User not found".to_owned() })
            }
        }
    }
}
