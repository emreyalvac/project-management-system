use async_trait::async_trait;
use domain::query::query::TQueryHandler;
use crate::queries::board_queries::get_all_board_query::GetAllBoardQuery;
use domain::board::board::Board;
use domain::common::not_found::NotFound;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};

pub struct GetAllBoardQueryHandler {}

#[async_trait]
impl TQueryHandler<GetAllBoardQuery, Vec<Board>, NotFound> for GetAllBoardQueryHandler {
    async fn get(&self) -> Result<Vec<Board>, NotFound> {
        let database: DatabaseConnection = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository: GenericRepository = GenericRepository { collection: "boards".to_owned(), connection: client };
                let result = repository.get_all::<Board>().await;
                Ok(result)
            }
            Err(_) => {
                Ok(Vec::new())
            }
        }
    }
}
