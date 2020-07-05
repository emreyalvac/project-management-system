use mongodb::{Client, bson::doc, options::{ClientOptions}};
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
        let mut options = ClientOptions::parse("mongodb://127.0.0.1:27017").await.unwrap();
        options.direct_connection = Some(true);
        match Client::with_options(options) {
            Ok(client) => {
                Ok(client)
            }
            Err(_) => {
                Err(DatabaseException { message: "Database connection failed".to_owned() })
            }
        }
    }

    async fn ping_connection(&self) -> Result<bool, bool> {
        match self.get_connection().await {
            Ok(connection) => {
                let ping = connection.database("project_management").run_command(doc! {"ping": 1}, None).await;
                match ping {
                    Ok(_) => {
                        return Ok(true);
                    }
                    Err(_) => {
                        return Err(false);
                    }
                }
            }
            Err(_) => {
                return Err(false);
            }
        }
    }
}
