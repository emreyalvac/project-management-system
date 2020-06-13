use crate::commands::board_commands::insert_card_to_board_command::InsertCardToBoardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;

pub struct InsertCardToBoardCommandHandler {
    pub command: InsertCardToBoardCommand
}

#[async_trait]
impl TCommandHandler<InsertCardToBoardCommand, CommandResponse> for InsertCardToBoardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository = GenericRepository { collection: "boards".to_owned(), connection: client };
                let board_id = self.command.card.board_id.clone();
                println!("{:?}", board_id);
                let card_id = self.command.card.card_id.clone();
                let execute = repository.update(doc! {"board_id": board_id}, doc! {"$push" => {"board_cards": card_id}}).await;
                match execute {
                    Ok(_) => return CommandResponse { command_type: CommandType::InsertCardToBoard, status: true, message: "OK".to_owned() },
                    Err(_) => return CommandResponse { command_type: CommandType::InsertCardToBoard, status: false, message: "Insert Failed".to_owned() }
                }
            }
            Err(_) => {
                return CommandResponse { command_type: CommandType::InsertCardToBoard, status: false, message: "Connection Failed".to_owned() };
            }
        }
    }
}
