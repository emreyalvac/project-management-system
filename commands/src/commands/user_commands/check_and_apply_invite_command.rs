use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct CheckAndApplyInviteCommand {
    pub board_id: String,
    pub user_id: String
}

impl TCommand<CommandResponse> for CheckAndApplyInviteCommand {}
