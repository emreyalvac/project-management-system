use crate::commands::user_commands::update_user_command::UpdateUserCommand;
use domain::command::command::TCommandHandler;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_response::CommandResponse;
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;

pub struct UpdateUserCommandHandler {
    pub command: UpdateUserCommand
}

#[async_trait]
impl TCommandHandler<UpdateUserCommand, CommandResponse> for UpdateUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database: DatabaseConnection = DatabaseConnection {};
        match database.get_connection().await {
            Ok(client) => {
                let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection: client };
                let user = self.command.user.clone();
                let result = repository.update(doc! {"id": user.user_id}, doc! {"$set": {"name": user.name, "surname": user.surname, "user_name": user.user_name}}).await;
                match result {
                    Ok(_) => {
                        CommandResponse { message: "Update OK".to_owned(), status: true, command_type: CommandType::UpdateUser }
                    }
                    Err(_) => {
                        CommandResponse { message: "Update Failed".to_owned(), status: false, command_type: CommandType::UpdateUser }
                    }
                }
            }
            Err(_) => {
                CommandResponse { message: "Connection Failed".to_owned(), status: false, command_type: CommandType::UpdateUser }
            }
        }
    }
}
