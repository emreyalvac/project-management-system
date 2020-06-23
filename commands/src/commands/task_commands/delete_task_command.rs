use domain::task::delete_task::DeleteTask;
use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct DeleteTaskCommand {
    pub task: DeleteTask
}

impl TCommand<CommandResponse> for DeleteTaskCommand {}
