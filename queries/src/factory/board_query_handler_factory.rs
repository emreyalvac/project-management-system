use crate::query_handlers::board_query_handlers::get_all_board_query_handler::GetAllBoardQueryHandler;

pub trait TBoardQueryHandlerFactory {
    fn build(&self) -> GetAllBoardQueryHandler;
}

pub struct BoardQueryHandlerFactory {}

impl TBoardQueryHandlerFactory for BoardQueryHandlerFactory {
    fn build(&self) -> GetAllBoardQueryHandler {
        GetAllBoardQueryHandler {}
    }
}
