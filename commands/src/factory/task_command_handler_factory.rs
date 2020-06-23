use crate::commands::task_commands::insert_task_command::{InsertTaskCommand};
use crate::command_handlers::task_command_handlers::insert_task_command_handler::{InsertTaskCommandHandler};
use crate::commands::task_commands::update_task_command::UpdateTaskCommand;
use crate::command_handlers::task_command_handlers::update_task_command_handler::UpdateTaskCommandHandler;
use crate::commands::task_commands::delete_task_command::DeleteTaskCommand;
use crate::command_handlers::task_command_handlers::delete_task_command_handler::DeleteTaskCommandHandler;

pub trait TTaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler;
    fn build_for_update(&self, command: UpdateTaskCommand) -> UpdateTaskCommandHandler;
    fn build_for_delete(&self, command: DeleteTaskCommand) -> DeleteTaskCommandHandler;
}

pub struct TaskCommandHandlerFactory {}

impl TTaskCommandHandlerFactory for TaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler {
        InsertTaskCommandHandler { command }
    }

    fn build_for_update(&self, command: UpdateTaskCommand) -> UpdateTaskCommandHandler {
        UpdateTaskCommandHandler { command }
    }

    fn build_for_delete(&self, command: DeleteTaskCommand) -> DeleteTaskCommandHandler {
        DeleteTaskCommandHandler { command }
    }
}
