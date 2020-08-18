use crate::commands::board_commands::insert_board_command::InsertBoardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;

use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use domain::board::insertable_board::InsertableBoard;
use mongodb::Client;

pub struct InsertBoardCommandHandler {
    pub command: InsertBoardCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<InsertBoardCommand, CommandResponse> for InsertBoardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { collection: "boards".to_owned(), connection };
        let result = repository.insert_generic::<InsertableBoard>(&self.command.board).await;
        return match result {
            Ok(_) => {
                CommandResponse { status: true, message: "OK".to_owned(), command_type: CommandType::InsertBoard }
            }
            Err(_) => {
                CommandResponse { message: "Insert Failed".to_owned(), status: false, command_type: CommandType::InsertBoard }
            }
        };
    }
}
