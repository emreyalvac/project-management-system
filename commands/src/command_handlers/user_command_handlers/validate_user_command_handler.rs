use crate::commands::user_commands::validate_user_command::ValidateUserCommand;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};
use domain::common::command_type::CommandType;
use domain::command::command::TCommandHandler;

pub struct ValidateUserCommandHandler {
    pub command: ValidateUserCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<ValidateUserCommand, CommandResponse> for ValidateUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
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
