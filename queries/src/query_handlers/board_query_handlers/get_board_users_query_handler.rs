use crate::queries::board_queries::get_board_users_query::GetBoardUsersQuery;
use domain::query::query::TQueryHandler;
use domain::board::board_users_aggregate::BoardUsersAggregate;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use async_trait::async_trait;
use domain::common::not_found::NotFound;
use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use bson::doc;

pub struct GetBoardUsersQueryHandler {
    pub query: GetBoardUsersQuery
}

#[async_trait]
impl TQueryHandler<GetBoardUsersQuery, BoardUsersAggregate, NotFound> for GetBoardUsersQueryHandler {
    async fn get(&self) -> Result<BoardUsersAggregate, NotFound> {
        let database = DatabaseConnection {};
        let connection = database.get_connection().await.ok().unwrap();
        let repository = GenericRepository { collection: "boards".to_owned(), connection };
        let board_id = self.query.board_id.clone();
        let match_query = doc! {"$match" => {"board_id": board_id}};
        let lookup_query = doc! {"$lookup" => {"from": "users", "localField": "board_id", "foreignField": "user_boards", "as": "users"}};
        let unwind_query = doc! {"$unwind": "$users"};
        let group_query = doc! {"$group" => {"_id": "$_id", "board": {"$first" => {"board_name": "$board_name", "board_id": "$board_id", "board_manager_user_id": "$board_manager_user_id"}}, "users": {"$push": "$users"}}};
        let result = repository.aggregate_one::<BoardUsersAggregate>(vec![match_query, lookup_query, unwind_query, group_query]).await;
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
