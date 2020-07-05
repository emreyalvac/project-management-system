use crate::commands::task_commands::assign_task_to_user_command::AssignTaskToUserCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};
use domain::task::task::Task;

pub struct AssignTaskToUserCommandHandler {
    pub command: AssignTaskToUserCommand,
    pub client: Client,
}

#[async_trait]
impl TCommandHandler<AssignTaskToUserCommand, CommandResponse> for AssignTaskToUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { connection, collection: "tasks".to_owned() };
        let task_id = &self.command.task_id;
        let user_id = &self.command.user_id;
        // Check user exist
        let check_is_user_exist = repository.get_by_custom_query::<Task>(doc! {"task_id": task_id, "task_assigned_users": {"$in": [user_id]}}).await;
        if check_is_user_exist.is_empty() {
            let result = repository.update(doc! {"task_id": task_id}, doc! {"$push": {"task_assigned_users": user_id}}).await;
            match result {
                Ok(_) => {
                    CommandResponse { status: true, message: "Assign OK".to_owned(), command_type: CommandType::AssignTaskToUser }
                }
                Err(_) => {
                    CommandResponse { status: false, message: "Assign Failed".to_owned(), command_type: CommandType::AssignTaskToUser }
                }
            }
        } else {
            CommandResponse { status: false, command_type: CommandType::AssignTaskToUser, message: "User already have this task".to_owned() }
        }
    }
}
