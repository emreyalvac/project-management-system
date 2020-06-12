use async_trait::async_trait;
use redis::{Client, Commands, Value, Connection};

#[async_trait]
pub trait TRedis {
    async fn get_connection(&self, url: String) -> Result<Connection, bool>;
    async fn set(&self, key: String, data: String) -> Result<bool, bool>;
    async fn get(&self, key: String) -> Result<String, bool>;
    async fn delete(&self, key: String) -> Result<bool, bool>;
}

pub struct Redis {}

#[async_trait]
impl TRedis for Redis {
    async fn get_connection(&self, url: String) -> Result<Connection, bool> {
        let client = match Client::open(url) {
            Ok(cli) => cli,
            Err(_) => return Err(false)
        };
        match client.get_connection() {
            Ok(con) => Ok(con),
            Err(_) => Err(false)
        }
    }

    async fn set(&self, key: String, data: String) -> Result<bool, bool> {
        let mut client = match self.get_connection("redis://127.0.0.1".to_owned()).await {
            Ok(client_) => client_,
            Err(_) => {
                return Err(false);
            }
        };
        match client.set::<String, String, String>(key, data) {
            Ok(result) => Ok(true),
            Err(_) => Err(false)
        }
    }

    async fn get(&self, key: String) -> Result<String, bool> {
        let mut client = match self.get_connection("redis://127.0.0.1".to_owned()).await {
            Ok(client) => client,
            Err(_) => return Err(false)
        };
        let value = client.get::<String, String>(key);
        match value {
            Ok(value_) => Ok(value_),
            Err(_) => Err(false)
        }
    }

    async fn delete(&self, key: String) -> Result<bool, bool> {
        let mut client = match self.get_connection("redis://127.0.0.1".to_owned()).await {
            Ok(client) => client,
            Err(_) => return Err(false)
        };
        let result = client.del::<String, Value>(key);
        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncWrite;
    use super::*;

    #[tokio::test]
    async fn redis_set() {
        let connection = Redis {};
        let handler = connection.set("KEY".to_owned(), "VALUE".to_owned()).await;
        assert_eq!(true, handler.unwrap())
    }

    #[tokio::test]
    async fn redis_del() {
        let connection = Redis {};
        let handler = connection.delete("KEY".to_owned()).await;
        assert_eq!(true, handler.unwrap())
    }
}
