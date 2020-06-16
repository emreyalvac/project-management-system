use crate::commands::task_commands::insert_task_command::{InsertTaskCommand};
use crate::command_handlers::task_command_handlers::insert_task_command_handler::{InsertTaskCommandHandler};

pub trait TTaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler;
}

pub struct TaskCommandHandlerFactory {}

impl TTaskCommandHandlerFactory for TaskCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertTaskCommand) -> InsertTaskCommandHandler {
        InsertTaskCommandHandler { command }
    }
}
