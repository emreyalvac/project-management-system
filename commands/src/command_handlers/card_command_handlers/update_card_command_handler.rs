use crate::commands::board_commands::update_card_command::UpdateCardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use async_trait::async_trait;
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};

pub struct UpdateCardCommandHandler {
    pub command: UpdateCardCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<UpdateCardCommand, CommandResponse> for UpdateCardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { collection: "cards".to_owned(), connection };
        let card = self.command.card.clone();
        let result = repository.update(doc! {"card_id": card.card_id}, doc! {"$set": {"card_name": card.card_name, "card_status": card.card_status}}).await;
        match result {
            Ok(_) => {
                CommandResponse { status: true, command_type: CommandType::UpdateCard, message: "Update OK".to_owned() }
            }
            Err(_) => {
                CommandResponse { message: "Update Failed".to_owned(), status: false, command_type: CommandType::UpdateCard }
            }
        }
    }
}
