use crate::command_handlers::user_command_handlers::insert_user_command_handler::InsertUserCommandHandler;
use crate::commands::user_commands::insert_user_command::InsertUserCommand;
use crate::commands::user_commands::validate_user_command::ValidateUserCommand;
use crate::command_handlers::user_command_handlers::validate_user_command_handler::ValidateUserCommandHandler;
use crate::commands::user_commands::insert_board_to_user_command::InsertBoardToUserCommand;
use crate::command_handlers::user_command_handlers::insert_board_to_user_command_handler::InsertBoardToUserCommandHandler;
use crate::commands::user_commands::check_and_apply_invite_command::CheckAndApplyInviteCommand;
use crate::command_handlers::user_command_handlers::check_and_apply_invite_command_handler::CheckAndApplyInviteCommandHandler;
use crate::commands::user_commands::update_user_command::UpdateUserCommand;
use crate::command_handlers::user_command_handlers::update_user_command_handler::UpdateUserCommandHandler;
use mongodb::Client;

pub trait TUserCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertUserCommand) -> InsertUserCommandHandler;
    fn build_for_validate(&self, command: ValidateUserCommand) -> ValidateUserCommandHandler;
    fn build_for_insert_board(&self, command: InsertBoardToUserCommand) -> InsertBoardToUserCommandHandler;
    fn build_for_check_and_apply_invite(&self, command: CheckAndApplyInviteCommand) -> CheckAndApplyInviteCommandHandler;
    fn build_for_update(&self, command: UpdateUserCommand) -> UpdateUserCommandHandler;
}

pub struct UserCommandHandlerFactory {
    pub client: Client
}

impl TUserCommandHandlerFactory for UserCommandHandlerFactory {
    fn build_for_insert(&self, command: InsertUserCommand) -> InsertUserCommandHandler {
        InsertUserCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_validate(&self, command: ValidateUserCommand) -> ValidateUserCommandHandler {
        ValidateUserCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_insert_board(&self, command: InsertBoardToUserCommand) -> InsertBoardToUserCommandHandler {
        InsertBoardToUserCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_check_and_apply_invite(&self, command: CheckAndApplyInviteCommand) -> CheckAndApplyInviteCommandHandler {
        CheckAndApplyInviteCommandHandler { command, client: self.client.to_owned() }
    }

    fn build_for_update(&self, command: UpdateUserCommand) -> UpdateUserCommandHandler {
        UpdateUserCommandHandler { command, client: self.client.to_owned() }
    }
}
