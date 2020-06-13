use crate::command_handlers::user_command_handlers::insert_user_command_handler::InsertUserCommandHandler;
use crate::commands::user_commands::insert_user_command::InsertUserCommand;
use crate::commands::user_commands::validate_user_command::ValidateUserCommand;
use crate::command_handlers::user_command_handlers::validate_user_command_handler::ValidateUserCommandHandler;

pub trait TUserCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertUserCommand) -> InsertUserCommandHandler;
    fn build_for_validate(&self, command: ValidateUserCommand) -> ValidateUserCommandHandler;
}

pub struct UserCommandHandlerFactory {}

impl TUserCommandHandlerFactory for UserCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertUserCommand) -> InsertUserCommandHandler {
        InsertUserCommandHandler { command }
    }

    fn build_for_validate(&self, command: ValidateUserCommand) -> ValidateUserCommandHandler {
        ValidateUserCommandHandler { command }
    }
}
