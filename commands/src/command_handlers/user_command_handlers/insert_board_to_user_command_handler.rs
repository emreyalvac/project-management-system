use crate::commands::user_commands::insert_board_to_user_command::InsertBoardToUserCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};


pub struct InsertBoardToUserCommandHandler {
    pub command: InsertBoardToUserCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<InsertBoardToUserCommand, CommandResponse> for InsertBoardToUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection };
        let user_id = self.command.user_board.user_id.clone();
        let board_id = self.command.user_board.board_id.clone();
        let handler = repository.update(doc! {"id": user_id}, doc! {"$push": {"user_boards": board_id}}).await;
        match handler {
            Ok(_) => {
                return CommandResponse { status: true, command_type: CommandType::InsertBoardToUser, message: "OK".to_owned() };
            }
            Err(err) => {
                return CommandResponse { status: false, command_type: CommandType::InsertBoardToUser, message: "Insert Failed".to_owned() };
            }
        }
    }
}
