use crate::commands::card_commands::insert_card_command::InsertCardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use async_trait::async_trait;
use domain::board::insert_card_to_board::InsertCardToBoard;
use mongodb::Client;

pub struct InsertCardCommandHandler {
    pub command: InsertCardCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<InsertCardCommand, CommandResponse> for InsertCardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository = GenericRepository { connection, collection: "cards".to_owned() };
        let card = self.command.card.clone();
        let handler = repository.insert_generic::<InsertCardToBoard>(&card).await;
        drop(card);
        match handler {
            Ok(_) => {
                return CommandResponse { command_type: CommandType::InsertCard, message: "OK".to_owned(), status: true };
            }
            Err(_) => {
                return CommandResponse { command_type: CommandType::InsertCard, message: "Insert Failed".to_owned(), status: false };
            }
        }
    }
}
