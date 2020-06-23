use async_trait::async_trait;
use domain::common::command_response::CommandResponse;
use commands::factory::card_command_hander_factory::{CardCommandHandlerFactory, TCardCommandHandlerFactory};
use commands::commands::card_commands::insert_card_command::InsertCardCommand;
use domain::command::command::TCommandHandler;
use domain::board::insert_card_to_board::InsertCardToBoard;
use domain::card::insert_task_to_card::InsertTask;
use crate::task_services::task::{TaskServices, TTaskServices};
use commands::commands::card_commands::insert_task_to_card_command::InsertTaskToCardCommand;
use domain::card::move_task_to_another_card::MoveTaskToAnotherCard;
use commands::commands::card_commands::move_task_to_another_card_command::MoveTaskToAnotherCardCommand;
use domain::card::update_card::UpdateCard;
use commands::commands::board_commands::update_card_command::UpdateCardCommand;

#[async_trait]
pub trait TCardServices {
    async fn insert_card(&self, card: InsertCardToBoard) -> Result<CommandResponse, CommandResponse>;
    async fn insert_task_to_card(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse>;
    async fn move_task_to_another_card(&self, task: MoveTaskToAnotherCard) -> Result<CommandResponse, CommandResponse>;
    async fn update_card(&self, card: UpdateCard) -> Result<CommandResponse, CommandResponse>;
}

pub struct CardServices {}

#[async_trait]
impl TCardServices for CardServices {
    async fn insert_card(&self, card: InsertCardToBoard) -> Result<CommandResponse, CommandResponse> {
        let factory = CardCommandHandlerFactory {};
        let mut handler = factory.build_for_insert(InsertCardCommand { card });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn insert_task_to_card(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse> {
        let card_task = task.clone();
        let task_services = TaskServices {};
        let task_insert = task_services.insert_task(task).await;
        let factory = CardCommandHandlerFactory {};
        let mut handler = factory.build_for_insert_task_to_card(InsertTaskToCardCommand { task: card_task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn move_task_to_another_card(&self, task: MoveTaskToAnotherCard) -> Result<CommandResponse, CommandResponse> {
        let factory = CardCommandHandlerFactory {};
        let mut handler = factory.build_for_move_task_to_another_card(MoveTaskToAnotherCardCommand { task });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn update_card(&self, card: UpdateCard) -> Result<CommandResponse, CommandResponse> {
        let factory = CardCommandHandlerFactory {};
        let mut handler = factory.build_for_update_card(UpdateCardCommand { card });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }
}
