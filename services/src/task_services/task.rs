use async_trait::async_trait;
use domain::card::insert_task_to_card::InsertTask;
use domain::common::command_response::CommandResponse;
use commands::factory::task_command_handler_factory::{TaskCommandHandlerFactory, TTaskCommandHandlerFactory};
use commands::commands::task_commands::insert_task_command::InsertTaskCommand;
use domain::command::command::TCommandHandler;

#[async_trait]
pub trait TTaskServices {
    async fn insert_task(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse>;
}

pub struct TaskServices {}

#[async_trait]
impl TTaskServices for TaskServices {
    async fn insert_task(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse> {
        let factory = TaskCommandHandlerFactory {};
        let mut handler = factory.build_for_insert(InsertTaskCommand { task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }
}
