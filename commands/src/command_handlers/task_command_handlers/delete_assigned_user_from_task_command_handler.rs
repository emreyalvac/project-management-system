use crate::commands::task_commands::delete_assigned_user_from_task_command::DeleteAssignedUserFromTaskCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;

use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};
use domain::common::command_type::CommandType;

pub struct DeleteAssignedUserFromTaskCommandHandler {
    pub command: DeleteAssignedUserFromTaskCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<DeleteAssignedUserFromTaskCommand, CommandResponse> for DeleteAssignedUserFromTaskCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { connection, collection: "tasks".to_owned() };
        let task_id = &self.command.task_id;
        let user_id = &self.command.user_id;
        let result = repository.update(doc! {"task_id": task_id}, doc! {"$pull": {"task_assigned_users": user_id}}).await;
        return match result {
            Ok(_) => {
                CommandResponse { status: true, message: "OK".to_owned(), command_type: CommandType::DeleteAssignedUserFromTask }
            }
            Err(_) => {
                CommandResponse { status: false, message: "Delete Failed".to_owned(), command_type: CommandType::DeleteAssignedUserFromTask }
            }
        };
    }
}
