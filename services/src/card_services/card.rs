use async_trait::async_trait;
use domain::common::command_response::CommandResponse;
use commands::factory::card_command_hander_factory::{CardCommandHandlerFactory, TCardCommandHandlerFactory};
use commands::commands::card_commands::insert_card_command::InsertCardCommand;
use domain::command::command::TCommandHandler;
use domain::board::insert_card_to_board::InsertCardToBoard;
use domain::card::insert_task_to_card::InsertTask;

#[async_trait]
pub trait TCardServices {
    async fn insert_card(&self, card: InsertCardToBoard) -> Result<CommandResponse, CommandResponse>;
    async fn insert_task_to_card(&self, task: InsertTask) -> Result<CommandResponse, CommandResponse>;
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
        unimplemented!()
    }
}
