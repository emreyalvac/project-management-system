use crate::commands::board_commands::insert_board_command::InsertBoardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use domain::board::insertable_board::InsertableBoard;

pub struct InsertBoardCommandHandler {
    pub command: InsertBoardCommand
}

#[async_trait]
impl TCommandHandler<InsertBoardCommand, CommandResponse> for InsertBoardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database: DatabaseConnection = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository: GenericRepository = GenericRepository { collection: "boards".to_owned(), connection: client };
                let result = repository.insert_generic::<InsertableBoard>(&self.command.board).await;
                match result {
                    Ok(_) => {
                        return CommandResponse { status: true, message: "OK".to_owned(), command_type: CommandType::InsertBoard };
                    }
                    Err(_) => {
                        return CommandResponse { message: "Insert Failed".to_owned(), status: false, command_type: CommandType::InsertBoard };
                    }
                }
            }
            Err(_) => {
                return CommandResponse { message: "Connection Failed".to_owned(), status: false, command_type: CommandType::InsertBoard };
            }
        }
    }
}
