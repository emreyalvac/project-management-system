use domain::common::not_found::NotFound;
use domain::common::found_type::FoundType;
use async_trait::async_trait;
use mongodb::{Client, bson};
use serde::{de::DeserializeOwned, Serialize};
use futures::stream::StreamExt;

#[async_trait]
pub trait TGenericRepository {
    async fn get_all<T>(&self) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send;
    async fn get_by_generic<T>(&self, column: String, value: String) -> Result<T, NotFound> where T: DeserializeOwned + 'static + Sized + Send;
    async fn insert_generic<T>(&self, data: &T) -> Result<bool, bool> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync;
    async fn aggregate<T>(&self, queries: Vec<bson::document::Document>) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync;
}

pub struct GenericRepository {
    pub connection: Client,
    pub collection: String,
}

#[async_trait]
impl TGenericRepository for GenericRepository {
    async fn get_all<T>(&self) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send {
        let mut data: Vec<T> = Vec::new();
        let db = self.connection.database("test");
        let mut cursor = db.collection(self.collection.as_str()).find(None, None).await.unwrap();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    match bson::from_bson::<T>(bson::Bson::Document(document)) {
                        Ok(doc) => {
                            data.push(doc)
                        }
                        Err(_) => {
                            return data;
                        }
                    }
                }
                Err(_) => {
                    return data;
                }
            }
        }
        data
    }

    async fn get_by_generic<T>(&self, column: String, value: String) -> Result<T, NotFound> where T: DeserializeOwned + 'static + Sized + Send {
        let db = self.connection.database("test");
        let cursor = db.collection(self.collection.as_str()).find_one(bson::doc! {column: value}, None).await.unwrap();
        match cursor {
            Some(doc) => {
                match bson::from_bson::<T>(bson::Bson::Document(doc)) {
                    Ok(document) => {
                        Ok(document)
                    }
                    Err(_) => {
                        Err(NotFound { message: "Not found".to_owned(), found_type: FoundType::News })
                    }
                }
            }
            None => {
                Err(NotFound { message: "Not found".to_owned(), found_type: FoundType::News })
            }
        }
    }

    async fn insert_generic<T>(&self, data: &T) -> Result<bool, bool> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync {
        let db = self.connection.database("test");
        let bson = bson::to_bson(&data).unwrap();
        let de_reference = bson.as_document().unwrap();
        let cloned = de_reference.clone();
        let cursor = db.collection(self.collection.as_str()).insert_one(cloned, None).await;
        match cursor {
            Ok(_) => Ok(true),
            Err(_) => Err(false)
        }
    }

    async fn aggregate<T>(&self, queries: Vec<bson::document::Document>) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync {
        let db = self.connection.database("test");
        let mut cursor = db.collection(self.collection.as_str()).aggregate(queries, None).await.unwrap();
        let mut data: Vec<T> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    match bson::from_bson::<T>(bson::Bson::Document(doc)) {
                        Ok(result_doc) => {
                            data.push(result_doc)
                        }
                        Err(_) => {
                            return data;
                        }
                    }
                }
                Err(_) => {
                    return data;
                }
            }
        }
        data
    }
}
