use crate::commands::user_commands::insert_board_to_user_command::InsertBoardToUserCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;

pub struct InsertBoardToUserCommandHandler {
    pub command: InsertBoardToUserCommand
}

#[async_trait]
impl TCommandHandler<InsertBoardToUserCommand, CommandResponse> for InsertBoardToUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database: DatabaseConnection = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection: client };
                let user_id = self.command.user_board.user_id.clone();
                let board_id = self.command.user_board.board_id.clone();
                let handler = repository.update(doc! {"id": user_id}, doc! {"$push" => {"user_boards": board_id}}).await;
                match handler {
                    Ok(_) => {
                        return CommandResponse { status: true, command_type: CommandType::InsertBoardToUser, message: "OK".to_owned() };
                    }
                    Err(err) => {
                        return CommandResponse { status: false, command_type: CommandType::InsertBoardToUser, message: "Insert Failed".to_owned() };
                    }
                }
            }
            Err(_) => {
                return CommandResponse { status: false, command_type: CommandType::InsertBoardToUser, message: "Connection Failed".to_owned() };
            }
        }
    }
}
