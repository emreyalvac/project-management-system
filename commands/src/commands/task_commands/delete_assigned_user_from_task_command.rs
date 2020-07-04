use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct DeleteAssignedUserFromTaskCommand {
    pub task_id: String,
    pub user_id: String,
}

impl TCommand<CommandResponse> for DeleteAssignedUserFromTaskCommand {}
