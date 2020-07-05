use crate::commands::task_commands::update_task_command::UpdateTaskCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;

use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};

pub struct UpdateTaskCommandHandler {
    pub command: UpdateTaskCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<UpdateTaskCommand, CommandResponse> for UpdateTaskCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { connection, collection: "tasks".to_owned() };
        let task = self.command.task.clone();
        let handler = repository.update(doc! {"task_id": task.task_id}, doc! {"$set": {"task_name": task.task_name, "task_start_date": task.task_start_date, "task_end_date": task.task_end_date, "task_description": task.task_description, "task_status": task.task_status}}).await;
        match handler {
            Ok(_) => {
                CommandResponse { status: true, message: "Update OK".to_owned(), command_type: CommandType::UpdateTask }
            }
            Err(_) => {
                CommandResponse { status: false, message: "Update Failed".to_owned(), command_type: CommandType::UpdateTask }
            }
        }
    }
}
