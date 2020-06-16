use crate::commands::card_commands::insert_card_command::InsertCardCommand;
use crate::command_handlers::card_command_handlers::insert_card_command_handler::InsertCardCommandHandler;
use crate::commands::card_commands::insert_task_to_card_command::InsertTaskToCardCommand;
use crate::command_handlers::card_command_handlers::insert_task_to_card_command_handler::InsertTaskToCardCommandHandler;

pub trait TCardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertCardCommand) -> InsertCardCommandHandler;
    fn build_for_insert_task_to_card(&self, command: InsertTaskToCardCommand) -> InsertTaskToCardCommandHandler;
}

pub struct CardCommandHandlerFactory {}

impl TCardCommandHandlerFactory for CardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertCardCommand) -> InsertCardCommandHandler {
        InsertCardCommandHandler { command }
    }

    fn build_for_insert_task_to_card(&self, command: InsertTaskToCardCommand) -> InsertTaskToCardCommandHandler {
        InsertTaskToCardCommandHandler { command }
    }
}
