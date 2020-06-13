use domain::common::command_response::CommandResponse;
use domain::command::command::TCommand;

pub struct ValidateUserCommand {
    pub id: String
}

impl TCommand<CommandResponse> for ValidateUserCommand {}
