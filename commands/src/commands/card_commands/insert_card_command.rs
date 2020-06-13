use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;
use domain::board::insert_card_to_board::InsertCardToBoard;

pub struct InsertCardCommand {
    pub card: InsertCardToBoard
}

impl TCommand<CommandResponse> for InsertCardCommand {}
