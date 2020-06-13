use domain::common::command_response::CommandResponse;
use domain::user::register::Register;
use domain::command::command::TCommand;

pub struct InsertUserCommand {
    pub user: Register
}

impl TCommand<CommandResponse> for InsertUserCommand {}
