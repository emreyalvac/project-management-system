use crate::queries::user_queries::get_user_boards_aggregate_query::GetUserBoardsAggregateQuery;
use domain::query::query::TQueryHandler;
use domain::board::board::Board;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;
use async_trait::async_trait;
use domain::aggregates::board_user_aggregate::BoardUserAggregate;

pub struct GetUserBoardsAggregateQueryHandler {
    pub query: GetUserBoardsAggregateQuery
}

#[async_trait]
impl TQueryHandler<GetUserBoardsAggregateQuery, BoardUserAggregate, BoardUserAggregate> for GetUserBoardsAggregateQueryHandler {
    async fn get(&self) -> Result<BoardUserAggregate, BoardUserAggregate> {
        let database = DatabaseConnection {};
        let connection = database.get_connection().await;
        match connection {
            Ok(client) => {
                let repository = GenericRepository { collection: "users".to_owned(), connection: client };
                let user_id = self.query.user_id.clone();
                let query_1 = doc! {"$match" => {"id": user_id}};
                let query_2 = doc! {"$lookup" => {"from": "boards", "localField": "user_boards", "foreignField": "board_id", "as": "boards"}};
                let query_3 = doc! {"$unwind" => {"path": "$boards", "preserveNullAndEmptyArrays": true}};
                let query_4 = doc! {"$group" => {"_id": "$_id", "boards" => {"$push" => {"board_id": "$boards.board_id", "board_name": "$boards.board_name", "board_manager_user_id": "$boards.board_manager_user_id"}}}};
                let handler = repository.aggregate_one::<BoardUserAggregate>(vec![query_1, query_2, query_3, query_4]).await;
                match handler {
                    Ok(data) => Ok(data),
                    Err(_) => Ok(BoardUserAggregate { boards: Vec::new() })
                }
            }
            Err(_) => {
                Ok(BoardUserAggregate { boards: Vec::new() })
            }
        }
    }
}
