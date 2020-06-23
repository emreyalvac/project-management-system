use domain::task::update_task::UpdateTask;
use domain::common::command_response::CommandResponse;
use domain::command::command::TCommand;

pub struct UpdateTaskCommand {
    pub task: UpdateTask
}

impl TCommand<CommandResponse> for UpdateTaskCommand {}
