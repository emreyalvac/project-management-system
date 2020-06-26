use domain::user::update_user::UpdateUser;
use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct UpdateUserCommand {
    pub user: UpdateUser
}

impl TCommand<CommandResponse> for UpdateUserCommand {}
