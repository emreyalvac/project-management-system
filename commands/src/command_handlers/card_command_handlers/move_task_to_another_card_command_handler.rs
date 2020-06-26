use crate::commands::card_commands::move_task_to_another_card_command::MoveTaskToAnotherCardCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;
use domain::common::command_type::CommandType;


pub struct MoveTaskToAnotherCardCommandHandler {
    pub command: MoveTaskToAnotherCardCommand
}

#[async_trait]
impl TCommandHandler<MoveTaskToAnotherCardCommand, CommandResponse> for MoveTaskToAnotherCardCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database: DatabaseConnection = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository = GenericRepository { collection: "cards".to_owned(), connection: client };
                // Remove Element
                let to_card_id = &self.command.task.to_card_id;
                let from_card_id = &self.command.task.from_card_id;
                let task_id = &self.command.task.task_id;
                let remove_element_query = doc! {"$pull": {"card_tasks": {"$in": [task_id]}}};
                let remove_element = repository.update(doc! {"card_id": from_card_id}, remove_element_query).await;
                let add_element_query = doc! {"$push": {"card_tasks": task_id}};
                let add_element = repository.update(doc! {"card_id": to_card_id}, add_element_query).await;
                match remove_element {
                    Ok(_) => {
                        match add_element {
                            Ok(_) => {
                                CommandResponse { message: "Update and Remove OK".to_owned(), command_type: CommandType::MoveTaskToAnotherCard, status: true }
                            }
                            Err(_) => {
                                CommandResponse { message: "Update Failed".to_owned(), command_type: CommandType::MoveTaskToAnotherCard, status: false }
                            }
                        }
                    }
                    Err(_) => {
                        CommandResponse { message: "Remove Failed".to_owned(), command_type: CommandType::MoveTaskToAnotherCard, status: false }
                    }
                }
            }
            Err(_) => {
                CommandResponse { message: "Connection Failed".to_owned(), command_type: CommandType::MoveTaskToAnotherCard, status: false }
            }
        }
    }
}
