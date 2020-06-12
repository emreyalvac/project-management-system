use mongodb::{Client, bson::doc};
use async_trait::async_trait;
use domain::database::database_exceptions::DatabaseException;

#[async_trait]
pub trait TDatabaseConnection {
    async fn get_connection(&self) -> Result<Client, DatabaseException>;
    async fn ping_connection(&self) -> Result<bool, bool>;
}

pub struct DatabaseConnection {}

#[async_trait]
impl TDatabaseConnection for DatabaseConnection {
    async fn get_connection(&self) -> Result<Client, DatabaseException> {
        match Client::with_uri_str("mongodb://127.0.0.1:27017").await {
            Ok(client) => {
                Ok(client)
            }
            Err(_) => {
                Err(DatabaseException { message: "Database connection failed".to_owned() })
            }
        }
    }

    async fn ping_connection(&self) -> Result<bool, bool> {
        let connection = self.get_connection().await.unwrap();
        let ping = connection.database("project_management").run_command(doc! {"ping": 1}, None).await;
        match ping {
            Ok(_) => {
                Ok(true)
            }
            Err(_) => {
                Err(false)
            }
        }
    }
}
