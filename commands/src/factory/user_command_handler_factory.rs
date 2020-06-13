use crate::command_handlers::user_command_handlers::insert_user_command_handler::InsertUserCommandHandler;
use crate::commands::user_commands::insert_user_command::InsertUserCommand;
use crate::commands::user_commands::validate_user_command::ValidateUserCommand;
use crate::command_handlers::user_command_handlers::validate_user_command_handler::ValidateUserCommandHandler;
use crate::commands::user_commands::insert_board_to_user_command::InsertBoardToUserCommand;
use crate::command_handlers::user_command_handlers::insert_board_to_user_command_handler::InsertBoardToUserCommandHandler;

pub trait TUserCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertUserCommand) -> InsertUserCommandHandler;
    fn build_for_validate(&self, command: ValidateUserCommand) -> ValidateUserCommandHandler;
    fn build_for_insert_board(&self, command: InsertBoardToUserCommand) -> InsertBoardToUserCommandHandler;
}

pub struct UserCommandHandlerFactory {}

impl TUserCommandHandlerFactory for UserCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertUserCommand) -> InsertUserCommandHandler {
        InsertUserCommandHandler { command }
    }

    fn build_for_validate(&self, command: ValidateUserCommand) -> ValidateUserCommandHandler {
        ValidateUserCommandHandler { command }
    }

    fn build_for_insert_board(&self, command: InsertBoardToUserCommand) -> InsertBoardToUserCommandHandler {
        InsertBoardToUserCommandHandler { command }
    }
}
