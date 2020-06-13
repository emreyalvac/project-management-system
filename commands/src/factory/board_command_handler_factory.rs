use crate::commands::board_commands::insert_board_command::InsertBoardCommand;
use crate::command_handlers::board_command_handlers::insert_board_command_handler::InsertBoardCommandHandler;

pub trait TBoardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertBoardCommand) -> InsertBoardCommandHandler;
}

pub struct BoardCommandHandlerFactory {}

impl TBoardCommandHandlerFactory for BoardCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertBoardCommand) -> InsertBoardCommandHandler {
        InsertBoardCommandHandler { command }
    }
}
