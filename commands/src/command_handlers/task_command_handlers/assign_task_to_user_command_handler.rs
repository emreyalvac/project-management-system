use crate::commands::task_commands::assign_task_to_user_command::AssignTaskToUserCommand;
use domain::command::command::TCommandHandler;
use domain::common::command_response::CommandResponse;
use async_trait::async_trait;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use domain::common::command_type::CommandType;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;

pub struct AssignTaskToUserCommandHandler {
    pub command: AssignTaskToUserCommand
}

#[async_trait]
impl TCommandHandler<AssignTaskToUserCommand, CommandResponse> for AssignTaskToUserCommandHandler {
    async fn execute(&mut self) -> CommandResponse {
        let database: DatabaseConnection = DatabaseConnection {};
        match database.get_connection().await {
            Ok(connection) => {
                let repository: GenericRepository = GenericRepository { connection, collection: "tasks".to_owned() };
                let task_id = &self.command.task_id;
                let user_id = &self.command.user_id;
                let result = repository.update(doc! {"task_id": task_id}, doc! {"$push": {"task_assigned_users": user_id}}).await;
                match result {
                    Ok(_) => {
                        CommandResponse { status: true, message: "Assign OK".to_owned(), command_type: CommandType::AssignTaskToUser }
                    }
                    Err(_) => {
                        CommandResponse { status: false, message: "Assign Failed".to_owned(), command_type: CommandType::AssignTaskToUser }
                    }
                }
            }
            Err(_) => {
                CommandResponse { status: false, message: "Connection Failed".to_owned(), command_type: CommandType::AssignTaskToUser }
            }
        }
    }
}
