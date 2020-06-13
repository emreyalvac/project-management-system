use domain::board::insert_card_to_board::InsertCardToBoard;
use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct InsertCardToBoardCommand {
    pub card: InsertCardToBoard
}

impl TCommand<CommandResponse> for InsertCardToBoardCommand {}
