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

#[async_trait]
pub trait TTaskServices {
    async fn insert_task(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse>;
    async fn update_task(&self, task: UpdateTask) -> Result<CommandResponse, CommandResponse>;
    async fn delete_task(&self, task: DeleteTask) -> Result<CommandResponse, CommandResponse>;
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

    async fn update_task(&self, task: UpdateTask) -> Result<CommandResponse, CommandResponse> {
        let factory = TaskCommandHandlerFactory {};
        let mut handler = factory.build_for_update(UpdateTaskCommand { task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn delete_task(&self, task: DeleteTask) -> Result<CommandResponse, CommandResponse> {
        let factory = TaskCommandHandlerFactory {};
        let mut handler = factory.build_for_delete(DeleteTaskCommand { task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }
}
