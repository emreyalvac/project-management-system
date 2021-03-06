use async_trait::async_trait;
use domain::aggregates::board_aggregate::BoardAggregate;
use domain::common::not_found::NotFound;
use queries::factory::board_query_handler_factory::{BoardQueryHandlerFactory, TBoardQueryHandlerFactory};
use domain::board::board_get_with_id::BoardGetWithId;
use queries::queries::board_queries::get_boards_as_aggregate_query::GetBoardAsAggregateQuery;
use domain::query::query::TQueryHandler;
use domain::board::insertable_board::InsertableBoard;
use domain::common::command_response::CommandResponse;
use commands::factory::board_command_handler_factory::{BoardCommandHandlerFactory, TBoardCommandHandlerFactory};
use commands::commands::board_commands::insert_board_command::InsertBoardCommand;
use domain::command::command::TCommandHandler;
use domain::board::insert_card_to_board::InsertCardToBoard;
use commands::commands::board_commands::insert_card_to_board_command::InsertCardToBoardCommand;
use crate::card_services::card::{CardServices, TCardServices};
use domain::board::board_users_aggregate::BoardUsersAggregate;
use queries::queries::board_queries::get_board_users_query::GetBoardUsersQuery;
use mongodb::Client;

#[async_trait]
pub trait TBoardServices {
    async fn get_board_as_aggregate(&self, board: BoardGetWithId) -> Result<BoardAggregate, NotFound>;
    async fn insert_board(&self, board: InsertableBoard) -> Result<CommandResponse, CommandResponse>;
    async fn insert_card_to_board(&self, card: InsertCardToBoard) -> Result<CommandResponse, CommandResponse>;
    async fn get_board_users(&self, board: BoardGetWithId) -> Result<BoardUsersAggregate, NotFound>;
}

pub struct BoardServices {
    pub client: Client
}

#[async_trait]
impl TBoardServices for BoardServices {
    async fn get_board_as_aggregate(&self, board: BoardGetWithId) -> Result<BoardAggregate, NotFound> {
        let factory = BoardQueryHandlerFactory { client: self.client.to_owned() };
        let handler = factory.build_for_aggregate(GetBoardAsAggregateQuery { board_id: board.board_id }).await;
        let result = handler.get().await;
        match result {
            Ok(board) => Ok(board),
            Err(err) => Err(err)
        }
    }

    async fn insert_board(&self, board: InsertableBoard) -> Result<CommandResponse, CommandResponse> {
        let factory = BoardCommandHandlerFactory { client: self.client.to_owned() };
        let mut handler = factory.build_for_insert(InsertBoardCommand { board });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn insert_card_to_board(&self, card: InsertCardToBoard) -> Result<CommandResponse, CommandResponse> {
        let factory = BoardCommandHandlerFactory { client: self.client.to_owned() };
        let card_clone = card.clone();
        let mut handler = factory.build_for_insert_card_to_board(InsertCardToBoardCommand { card });
        let result = handler.execute().await;
        if result.status {
            let card_services = CardServices { client: self.client.to_owned() };
            let _card_insert_result = card_services.insert_card(card_clone).await;
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn get_board_users(&self, board: BoardGetWithId) -> Result<BoardUsersAggregate, NotFound> {
        let factory = BoardQueryHandlerFactory { client: self.client.to_owned() };
        let handler = factory.build_for_get_board_users(GetBoardUsersQuery { board_id: board.board_id }).await;
        let result = handler.get().await;
        match result {
            Ok(res) => {
                Ok(res)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}
