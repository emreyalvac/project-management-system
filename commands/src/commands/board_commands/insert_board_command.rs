use domain::board::insertable_board::InsertableBoard;
use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct InsertBoardCommand {
    pub board: InsertableBoard
}

impl TCommand<CommandResponse> for InsertBoardCommand {}
