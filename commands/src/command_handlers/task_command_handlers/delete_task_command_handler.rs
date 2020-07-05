use crate::commands::task_commands::delete_task_command::DeleteTaskCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;

use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};
use domain::common::command_type::CommandType;

pub struct DeleteTaskCommandHandler {
    pub command: DeleteTaskCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<DeleteTaskCommand, CommandResponse> for DeleteTaskCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { collection: "tasks".to_owned(), connection };
        let task_id = &self.command.task.task_id;
        let handler = repository.delete_one(doc! {"task_id": task_id}).await;
        match handler {
            Ok(_) => {
                CommandResponse { status: true, message: "Delete OK".to_owned(), command_type: CommandType::DeleteTask }
            }
            Err(_) => {
                CommandResponse { status: false, message: "Delete Failed".to_owned(), command_type: CommandType::DeleteTask }
            }
        }
    }
}
