use async_trait::async_trait;
use domain::board::board::Board;
use domain::common::not_found::NotFound;
use queries::factory::board_query_handler_factory::{BoardQueryHandlerFactory, TBoardQueryHandlerFactory};
use domain::query::query::TQueryHandler;

#[async_trait]
pub trait TBoardServices {
    async fn get_all(&self) -> Result<Vec<Board>, NotFound>;
}

pub struct BoardServices {}

#[async_trait]
impl TBoardServices for BoardServices {
    async fn get_all(&self) -> Result<Vec<Board>, NotFound> {
        let factory = BoardQueryHandlerFactory {};
        let query = factory.build();
        let result = query.get().await;
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
