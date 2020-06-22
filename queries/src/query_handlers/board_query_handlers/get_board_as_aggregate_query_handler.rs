use async_trait::async_trait;
use crate::queries::board_queries::get_boards_as_aggregate_query::GetBoardAsAggregateQuery;
use domain::query::query::TQueryHandler;
use domain::aggregates::board_aggregate::BoardAggregate;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;
use domain::common::not_found::NotFound;
use std::collections::HashMap;

pub struct GetBoardAsAggregateQueryHandler {
    pub query: GetBoardAsAggregateQuery
}

#[async_trait]
impl TQueryHandler<GetBoardAsAggregateQuery, BoardAggregate, NotFound> for GetBoardAsAggregateQueryHandler {
    async fn get(&self) -> Result<BoardAggregate, NotFound> {
        let database = DatabaseConnection {};
        let connection = database.get_connection().await.ok().unwrap();
        let repository: GenericRepository = GenericRepository { collection: "boards".to_owned(), connection };
        let board_id = &self.query.board_id;
        let query_1 = doc! {"$match" => {"board_id": board_id.as_str()}};
        let query_2 = doc! {"$lookup" => {"from": "cards", "localField": "board_cards", "foreignField": "card_id", "as": "cards"}};
        let query_3 = doc! {"$unwind" => {"path": "$cards", "preserveNullAndEmptyArrays": true}};
        let query_4 = doc! {"$lookup" => {"from": "tasks", "localField": "cards.card_tasks", "foreignField": "task_id", "as": "tasks"}};
        let query_5 = doc! {"$group" => {"_id": "$_id", "board" => {"$first" => {"board_id": "$board_id", "board_name": "$board_name", "board_manager_user_id": "$board_manager_user_id", "board_status": "$board_status"}}, "cards" => {"$push" => {"card_id": "$cards.card_id", "card_name": "$cards.card_name", "tasks": "$tasks"}}}};
        let query_6 = doc! {"$project" => {"board": "$board", "cards" => {"$cond" => [{"$ne" => [{"$ifNull" => [{"$arrayElemAt" => ["$cards.card_id", 0]}, []]}, []]}, "$cards", []]}}};
        let query = repository.aggregate_one::<BoardAggregate>(vec![query_1, query_2, query_3, query_4, query_5, query_6]).await;
        match query {
            Ok(result) => {
                Ok(result)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}
