use crate::commands::card_commands::insert_task_to_card_command::InsertTaskToCardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;

use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};
use async_trait::async_trait;

pub struct InsertTaskToCardCommandHandler {
    pub command: InsertTaskToCardCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<InsertTaskToCardCommand, CommandResponse> for InsertTaskToCardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository = GenericRepository { collection: "cards".to_owned(), connection };
        let card_id = &self.command.task.card_id;
        let task_id = &self.command.task.task_id;
        let update = repository.update(doc! {"card_id": card_id}, doc! {"$push": {"card_tasks":  task_id}}).await;
        match update {
            Ok(_) => {
                CommandResponse { message: "OK".to_owned(), status: true, command_type: CommandType::InsertTaskToCard }
            }
            Err(_) => {
                CommandResponse { message: "Insert Failed".to_owned(), status: false, command_type: CommandType::InsertTaskToCard }
            }
        }
    }
}
