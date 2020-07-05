use crate::queries::user_queries::check_user_board_query::CheckUserBoardQuery;
use domain::query::query::TQueryHandler;
use async_trait::async_trait;

use data_access::generic_repository::generic_repository::{GenericRepository, TGenericRepository};
use mongodb::{bson::doc, Client};
use domain::user::user::User;

pub struct CheckUserBoardQueryHandler {
    pub query: CheckUserBoardQuery,
    pub client: Client,
}

#[async_trait]
impl TQueryHandler<CheckUserBoardQuery, bool, bool> for CheckUserBoardQueryHandler {
    async fn get(&self) -> Result<bool, bool> {
        let connection = self.client.to_owned();
        let repository: GenericRepository = GenericRepository { collection: "users".to_owned(), connection };
        let board_id = &self.query.board_id;
        let user_id = &self.query.user_id;
        let query = doc! {"$match": {"user_boards": board_id, "id": user_id}};
        let handler = repository.aggregate_one::<User>(vec![query]).await;
        match handler {
            Ok(_) => {
                Err(false)
            }
            Err(_) => {
                Ok(true)
            }
        }
    }
}
