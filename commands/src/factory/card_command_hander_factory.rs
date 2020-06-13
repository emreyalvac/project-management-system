use crate::commands::card_commands::insert_card_command::InsertCardCommand;
use crate::command_handlers::card_command_handlers::insert_card_command_handler::InsertCardCommandHandler;

pub trait TCardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertCardCommand) -> InsertCardCommandHandler;
}

pub struct CardCommandHandlerFactory {}

impl TCardCommandHandlerFactory for CardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertCardCommand) -> InsertCardCommandHandler {
        InsertCardCommandHandler { command }
    }
}
