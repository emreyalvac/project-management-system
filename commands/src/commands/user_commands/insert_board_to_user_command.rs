use domain::common::command_response::CommandResponse;
use domain::command::command::TCommand;
use domain::user::insert_board_to_user::InsertBoardToUser;

pub struct InsertBoardToUserCommand {
    pub user_board: InsertBoardToUser
}

impl TCommand<CommandResponse> for InsertBoardToUserCommand {}
