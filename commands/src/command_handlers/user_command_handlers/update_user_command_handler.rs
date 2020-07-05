use crate::commands::user_commands::update_user_command::UpdateUserCommand;
use domain::command::command::TCommandHandler;
use async_trait::async_trait;
use domain::common::command_response::CommandResponse;
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};

pub struct UpdateUserCommandHandler {
    pub command: UpdateUserCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<UpdateUserCommand, CommandResponse> for UpdateUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection };
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
}
