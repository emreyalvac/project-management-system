use domain::card::insert_task_to_card::{InsertTask};
use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct InsertTaskCommand {
    pub task: InsertTask
}

impl TCommand<CommandResponse> for InsertTaskCommand {}
