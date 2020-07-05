use async_trait::async_trait;
use domain::card::insert_task_to_card::InsertTask;
use domain::common::command_response::CommandResponse;
use commands::factory::task_command_handler_factory::{TaskCommandHandlerFactory, TTaskCommandHandlerFactory};
use commands::commands::task_commands::insert_task_command::InsertTaskCommand;
use domain::command::command::TCommandHandler;
use domain::task::update_task::UpdateTask;
use commands::commands::task_commands::update_task_command::UpdateTaskCommand;
use domain::task::delete_task::DeleteTask;
use commands::commands::task_commands::delete_task_command::DeleteTaskCommand;
use domain::task::assign_task_to_user::AssignTaskToUser;
use commands::commands::task_commands::assign_task_to_user_command::AssignTaskToUserCommand;
use crate::user_services::user::{UserServices, TUserServices};
use domain::user::user_get_by_id::UserGetById;
use domain::common::command_type::CommandType;
use domain::task::delete_assigned_user_from_task::DeleteAssignedUserFromTask;
use commands::commands::task_commands::delete_assigned_user_from_task_command::DeleteAssignedUserFromTaskCommand;
use mongodb::Client;

#[async_trait]
pub trait TTaskServices {
    async fn insert_task(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse>;
    async fn update_task(&self, task: UpdateTask) -> Result<CommandResponse, CommandResponse>;
    async fn delete_task(&self, task: DeleteTask) -> Result<CommandResponse, CommandResponse>;
    async fn assign_task_to_user(&self, task: AssignTaskToUser) -> Result<CommandResponse, CommandResponse>;
    async fn delete_assigned_user_from_task(&self, task: DeleteAssignedUserFromTask) -> Result<CommandResponse, CommandResponse>;
}

pub struct TaskServices {
    pub client: Client
}

#[async_trait]
impl TTaskServices for TaskServices {
    async fn insert_task(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse> {
        let factory = TaskCommandHandlerFactory { client: self.client.to_owned() };
        let mut handler = factory.build_for_insert(InsertTaskCommand { task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn update_task(&self, task: UpdateTask) -> Result<CommandResponse, CommandResponse> {
        let factory = TaskCommandHandlerFactory { client: self.client.to_owned() };
        let mut handler = factory.build_for_update(UpdateTaskCommand { task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn delete_task(&self, task: DeleteTask) -> Result<CommandResponse, CommandResponse> {
        let factory = TaskCommandHandlerFactory { client: self.client.to_owned() };
        let mut handler = factory.build_for_delete(DeleteTaskCommand { task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn assign_task_to_user(&self, task: AssignTaskToUser) -> Result<CommandResponse, CommandResponse> {
        let client_clone = self.client.clone();
        let user_services = UserServices { client: client_clone };
        let user_id = task.user_id.clone();
        let user_found_result = user_services.get_by_id(UserGetById { user_id }).await;
        // User Found Result
        match user_found_result {
            Ok(_) => {
                let factory = TaskCommandHandlerFactory { client: self.client.to_owned() };
                let mut handler = factory.build_for_assign_task_to_user(AssignTaskToUserCommand { user_id: task.user_id, task_id: task.task_id });
                let result = handler.execute().await;
                if result.status {
                    return Ok(result);
                } else {
                    return Err(result);
                }
            }
            Err(_) => {
                return Err(CommandResponse { status: false, command_type: CommandType::AssignTaskToUser, message: "User Not Found".to_owned() });
            }
        }
    }

    async fn delete_assigned_user_from_task(&self, task: DeleteAssignedUserFromTask) -> Result<CommandResponse, CommandResponse> {
        let factory = TaskCommandHandlerFactory { client: self.client.to_owned() };
        let mut handler = factory.build_for_delete_assigned_user_from_task(DeleteAssignedUserFromTaskCommand { user_id: task.user_id, task_id: task.task_id });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }
}
