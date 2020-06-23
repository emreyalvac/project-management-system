use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;
use domain::card::update_card::UpdateCard;

pub struct UpdateCardCommand {
    pub card: UpdateCard
}

impl TCommand<CommandResponse> for UpdateCardCommand {}
