use crate::commands::board_commands::insert_card_to_board_command::InsertCardToBoardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;

use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};

pub struct InsertCardToBoardCommandHandler {
    pub command: InsertCardToBoardCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<InsertCardToBoardCommand, CommandResponse> for InsertCardToBoardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository = GenericRepository { collection: "boards".to_owned(), connection };
        let board_id = self.command.card.board_id.clone();
        let card_id = self.command.card.card_id.clone();
        let execute = repository.update(doc! {"board_id": board_id}, doc! {"$push": {"board_cards": card_id}}).await;
        match execute {
            Ok(_) => return CommandResponse { command_type: CommandType::InsertCardToBoard, status: true, message: "OK".to_owned() },
            Err(_) => return CommandResponse { command_type: CommandType::InsertCardToBoard, status: false, message: "Insert Failed".to_owned() }
        }
    }
}
