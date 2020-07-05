use crate::commands::card_commands::insert_card_command::InsertCardCommand;
use crate::command_handlers::card_command_handlers::insert_card_command_handler::InsertCardCommandHandler;
use crate::commands::card_commands::insert_task_to_card_command::InsertTaskToCardCommand;
use crate::command_handlers::card_command_handlers::insert_task_to_card_command_handler::InsertTaskToCardCommandHandler;
use crate::commands::card_commands::move_task_to_another_card_command::MoveTaskToAnotherCardCommand;
use crate::command_handlers::card_command_handlers::move_task_to_another_card_command_handler::MoveTaskToAnotherCardCommandHandler;
use crate::commands::board_commands::update_card_command::UpdateCardCommand;
use crate::command_handlers::card_command_handlers::update_card_command_handler::UpdateCardCommandHandler;
use mongodb::Client;

pub trait TCardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertCardCommand) -> InsertCardCommandHandler;
    fn build_for_insert_task_to_card(&self, command: InsertTaskToCardCommand) -> InsertTaskToCardCommandHandler;
    fn build_for_move_task_to_another_card(&self, command: MoveTaskToAnotherCardCommand) -> MoveTaskToAnotherCardCommandHandler;
    fn build_for_update_card(&self, command: UpdateCardCommand) -> UpdateCardCommandHandler;
}

pub struct CardCommandHandlerFactory {
    pub client: Client
}

impl TCardCommandHandlerFactory for CardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertCardCommand) -> InsertCardCommandHandler {
        InsertCardCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_insert_task_to_card(&self, command: InsertTaskToCardCommand) -> InsertTaskToCardCommandHandler {
        InsertTaskToCardCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_move_task_to_another_card(&self, command: MoveTaskToAnotherCardCommand) -> MoveTaskToAnotherCardCommandHandler {
        MoveTaskToAnotherCardCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_update_card(&self, command: UpdateCardCommand) -> UpdateCardCommandHandler {
        UpdateCardCommandHandler { command, client: self.client.to_owned() }
    }
}
