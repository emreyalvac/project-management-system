use crate::commands::task_commands::insert_task_command::{InsertTaskCommand};
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use domain::card::insert_task_to_card::{InsertTask};
use async_trait::async_trait;

pub struct InsertTaskCommandHandler {
    pub command: InsertTaskCommand
}

#[async_trait]
impl TCommandHandler<InsertTaskCommand, CommandResponse> for InsertTaskCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository: GenericRepository = GenericRepository { collection: "tasks".to_owned(), connection: client };
                let handler = repository.insert_generic::<InsertTask>(&self.command.task).await;
                match handler {
                    Ok(_) => {
                        CommandResponse { message: "OK".to_owned(), status: true, command_type: CommandType::InsertTaskToCard }
                    }
                    Err(_) => {
                        CommandResponse { message: "Insert Failed".to_owned(), status: false, command_type: CommandType::InsertTaskToCard }
                    }
                }
            }
            Err(_) => {
                CommandResponse { message: "Connection Failed".to_owned(), status: false, command_type: CommandType::InsertTaskToCard }
            }
        }
    }
}