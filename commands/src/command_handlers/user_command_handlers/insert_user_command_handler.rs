use domain::common::command_response::CommandResponse;
use domain::common::command_type::CommandType;
use async_trait::async_trait;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use crate::commands::user_commands::insert_user_command::InsertUserCommand;
use domain::user::register::Register;
use domain::command::command::TCommandHandler;
use mongodb::Client;

pub struct InsertUserCommandHandler {
    pub command: InsertUserCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<InsertUserCommand, CommandResponse> for InsertUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection };
        let mut user_data = &mut self.command.user;
        let result = repository.insert_generic::<Register>(user_data).await.unwrap();
        if result {
            CommandResponse { status: true, message: "User Insert".to_owned(), command_type: CommandType::UserInsert }
        } else {
            CommandResponse { status: false, message: "User Insert".to_owned(), command_type: CommandType::UserInsert }
        }
    }
}
