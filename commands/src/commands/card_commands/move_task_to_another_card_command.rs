use domain::card::move_task_to_another_card::MoveTaskToAnotherCard;
use domain::command::command::TCommand;
use domain::common::command_response::CommandResponse;

pub struct MoveTaskToAnotherCardCommand {
    pub task: MoveTaskToAnotherCard
}

impl TCommand<CommandResponse> for MoveTaskToAnotherCardCommand {}
