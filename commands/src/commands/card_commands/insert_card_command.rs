use domain::card::insert_card::InsertCard;
use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct InsertCardCommand {
    pub card: InsertCard
}

impl TCommand<CommandResponse> for InsertCardCommand {}
