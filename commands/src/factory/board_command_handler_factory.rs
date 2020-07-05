use crate::commands::board_commands::insert_board_command::InsertBoardCommand;
use crate::command_handlers::board_command_handlers::insert_board_command_handler::InsertBoardCommandHandler;
use crate::commands::board_commands::insert_card_to_board_command::InsertCardToBoardCommand;
use crate::command_handlers::board_command_handlers::insert_card_to_board_command_handler::InsertCardToBoardCommandHandler;
use mongodb::Client;

pub trait TBoardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertBoardCommand) -> InsertBoardCommandHandler;
    fn build_for_insert_card_to_board(&self, command: InsertCardToBoardCommand) -> InsertCardToBoardCommandHandler;
}

pub struct BoardCommandHandlerFactory {
    pub client: Client
}

impl TBoardCommandHandlerFactory for BoardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertBoardCommand) -> InsertBoardCommandHandler {
        InsertBoardCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_insert_card_to_board(&self, command: InsertCardToBoardCommand) -> InsertCardToBoardCommandHandler {
        InsertCardToBoardCommandHandler { command, client: self.client.to_owned() }
    }
}
