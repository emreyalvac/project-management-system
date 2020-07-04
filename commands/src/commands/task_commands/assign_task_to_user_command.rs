use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct AssignTaskToUserCommand {
    pub user_id: String,
    pub task_id: String,
}

impl TCommand<CommandResponse> for AssignTaskToUserCommand {}
