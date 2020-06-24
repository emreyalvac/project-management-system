use crate::commands::user_commands::check_and_apply_invite_command::CheckAndApplyInviteCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;

pub struct CheckAndApplyInviteCommandHandler {
    pub command: CheckAndApplyInviteCommand
}

#[async_trait]
impl TCommandHandler<CheckAndApplyInviteCommand, CommandResponse> for CheckAndApplyInviteCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database: DatabaseConnection = DatabaseConnection {};
        match database.get_connection().await {
            Ok(client) => {
                let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection: client };
                let user_id = &self.command.user_id;
                let board_id = &self.command.board_id;
                let result = repository.update(doc! {"id": user_id}, doc! {"$push" => {"user_boards":  board_id}}).await;
                match result {
                    Ok(_) => {
                        CommandResponse { status: true, message: "Update OK".to_owned(), command_type: CommandType::CheckAndApplyInvite }
                    }
                    Err(_) => {
                        CommandResponse { status: false, message: "Update Failed".to_owned(), command_type: CommandType::CheckAndApplyInvite }
                    }
                }
            }
            Err(_) => {
                CommandResponse { status: false, message: "Connection Failed".to_owned(), command_type: CommandType::CheckAndApplyInvite }
            }
        }
    }
}
