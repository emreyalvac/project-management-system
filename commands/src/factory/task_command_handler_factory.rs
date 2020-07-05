use crate::commands::task_commands::insert_task_command::{InsertTaskCommand};
use crate::command_handlers::task_command_handlers::insert_task_command_handler::{InsertTaskCommandHandler};
use crate::commands::task_commands::update_task_command::UpdateTaskCommand;
use crate::command_handlers::task_command_handlers::update_task_command_handler::UpdateTaskCommandHandler;
use crate::commands::task_commands::delete_task_command::DeleteTaskCommand;
use crate::command_handlers::task_command_handlers::delete_task_command_handler::DeleteTaskCommandHandler;
use crate::commands::task_commands::assign_task_to_user_command::AssignTaskToUserCommand;
use crate::command_handlers::task_command_handlers::assign_task_to_user_command_handler::AssignTaskToUserCommandHandler;
use crate::commands::task_commands::delete_assigned_user_from_task_command::DeleteAssignedUserFromTaskCommand;
use crate::command_handlers::task_command_handlers::delete_assigned_user_from_task_command_handler::DeleteAssignedUserFromTaskCommandHandler;
use mongodb::Client;

pub trait TTaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler;
    fn build_for_update(&self, command: UpdateTaskCommand) -> UpdateTaskCommandHandler;
    fn build_for_delete(&self, command: DeleteTaskCommand) -> DeleteTaskCommandHandler;
    fn build_for_assign_task_to_user(&self, command: AssignTaskToUserCommand) -> AssignTaskToUserCommandHandler;
    fn build_for_delete_assigned_user_from_task(&self, command: DeleteAssignedUserFromTaskCommand) -> DeleteAssignedUserFromTaskCommandHandler;
}

pub struct TaskCommandHandlerFactory {
    pub client: Client
}

impl TTaskCommandHandlerFactory for TaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler {
        InsertTaskCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_update(&self, command: UpdateTaskCommand) -> UpdateTaskCommandHandler {
        UpdateTaskCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_delete(&self, command: DeleteTaskCommand) -> DeleteTaskCommandHandler {
        DeleteTaskCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_assign_task_to_user(&self, command: AssignTaskToUserCommand) -> AssignTaskToUserCommandHandler {
        AssignTaskToUserCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_delete_assigned_user_from_task(&self, command: DeleteAssignedUserFromTaskCommand) -> DeleteAssignedUserFromTaskCommandHandler {
        DeleteAssignedUserFromTaskCommandHandler { command, client: self.client.to_owned() }
    }
}
