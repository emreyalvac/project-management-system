use crate::commands::task_commands::insert_task_command::{InsertTaskCommand};
use crate::command_handlers::task_command_handlers::insert_task_command_handler::{InsertTaskCommandHandler};
use crate::commands::task_commands::update_task_command::UpdateTaskCommand;
use crate::command_handlers::task_command_handlers::update_task_command_handler::UpdateTaskCommandHandler;

pub trait TTaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler;
    fn build_for_update(&self, command: UpdateTaskCommand) -> UpdateTaskCommandHandler;
}

pub struct TaskCommandHandlerFactory {}

impl TTaskCommandHandlerFactory for TaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler {
        InsertTaskCommandHandler { command }
    }

    fn build_for_update(&self, command: UpdateTaskCommand) -> UpdateTaskCommandHandler {
        UpdateTaskCommandHandler { command }
    }
}
