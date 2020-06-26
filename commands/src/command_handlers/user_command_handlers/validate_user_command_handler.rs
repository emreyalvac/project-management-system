use crate::commands::user_commands::validate_user_command::ValidateUserCommand;
use domain::common::command_response::CommandResponse;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use async_trait::async_trait;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;
use domain::common::command_type::CommandType;
use domain::command::command::TCommandHandler;

pub struct ValidateUserCommandHandler {
    pub command: ValidateUserCommand
}

#[async_trait]
impl TCommandHandler<ValidateUserCommand, CommandResponse> for ValidateUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database: DatabaseConnection = DatabaseConnection {};
        let connection = database.get_connection().await.ok().unwrap();
        let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection };
        let id = self.command.id.clone();
        let filter = doc! {"id": id};
        let data = doc! {"$set": {"is_validate": true}};
        let result = repository.update(filter, data).await;
        match result {
            Ok(_) => {
                CommandResponse { message: "User Validation".to_owned(), command_type: CommandType::ValidateUser, status: true }
            }
            Err(_) => {
                CommandResponse { message: "User Validation".to_owned(), command_type: CommandType::ValidateUser, status: true }
            }
        }
    }
}
