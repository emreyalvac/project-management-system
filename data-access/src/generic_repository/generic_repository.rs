use domain::common::not_found::NotFound;
use domain::common::found_type::FoundType;
use async_trait::async_trait;
use mongodb::{Client, bson::{Document, doc, Bson, from_bson, to_bson}};
use serde::{de::DeserializeOwned};
use futures::stream::StreamExt;
use serde::{Serialize};
use std::fmt::Debug;

// TODO: Add aggregate_one function

#[async_trait]
pub trait TGenericRepository {
    async fn get_all<T>(&self) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send;
    async fn get_by_generic<T>(&self, column: String, value: String) -> Result<T, NotFound> where T: DeserializeOwned + 'static + Sized + Send;
    async fn insert_generic<T>(&self, data: &T) -> Result<bool, bool> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync;
    async fn aggregate<T>(&self, queries: Vec<Document>) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync;
    async fn aggregate_one<T>(&self, queries: Vec<Document>) -> Result<T, NotFound> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync + Debug;
    async fn update(&self, filter: Document, data: Document) -> Result<bool, bool>;
    async fn delete_one(&self, filter: Document) -> Result<bool, bool>;
    async fn get_by_custom_query<T>(&self, filter: Document) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send;
}

pub struct GenericRepository {
    pub connection: Client,
    pub collection: String,
}

#[async_trait]
impl TGenericRepository for GenericRepository {
    async fn get_all<T>(&self) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send {
        let mut data: Vec<T> = Vec::new();
        let db = self.connection.database("project_management");
        let mut cursor = db.collection(self.collection.as_str()).find(None, None).await.unwrap();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    match from_bson::<T>(Bson::Document(document)) {
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
        let db = self.connection.database("project_management");
        let cursor = db.collection(self.collection.as_str()).find_one(doc! {column: value}, None).await.unwrap();
        match cursor {
            Some(doc) => {
                match from_bson::<T>(Bson::Document(doc)) {
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
        let db = self.connection.database("project_management");
        let bson = to_bson(&data).unwrap();
        let de_reference = bson.as_document().unwrap();
        let cloned = de_reference.clone();
        let cursor = db.collection(self.collection.as_str()).insert_one(cloned, None).await;
        match cursor {
            Ok(_) => Ok(true),
            Err(_) => Err(false)
        }
    }

    async fn aggregate<T>(&self, queries: Vec<Document>) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync {
        let db = self.connection.database("project_management");
        let mut cursor = db.collection(self.collection.as_str()).aggregate(queries, None).await.unwrap();
        let mut data: Vec<T> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    match from_bson::<T>(Bson::Document(doc)) {
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

    async fn aggregate_one<T>(&self, queries: Vec<Document>) -> Result<T, NotFound> where T: DeserializeOwned + 'static + Sized + Send + Serialize + Sync + Debug {
        let db = self.connection.database("project_management");
        let mut cursor = db.collection(self.collection.as_str()).aggregate(queries, None).await;
        let mut data: Option<T> = None;
        match cursor {
            Ok(mut result) => {
                while let Some(doc) = result.next().await {
                    match doc {
                        Ok(res) => {
                            match from_bson(Bson::Document(res)) {
                                Ok(result) => {
                                    data = Some(result);
                                    break;
                                }
                                Err(err) => {
                                    println!("Aggregate One Encode Error -> {:?}", err);
                                    return Err(NotFound { message: "Decoder Error".to_owned(), found_type: FoundType::News });
                                }
                            }
                        }
                        Err(err) => {
                            println!("Err {:?}", err);
                            return Err(NotFound { message: "Not found _1".to_owned(), found_type: FoundType::News });
                        }
                    }
                }
            }
            Err(_) => return Err(NotFound { message: "Not found _2".to_owned(), found_type: FoundType::News })
        }
        match data {
            Some(result) => Ok(result),
            None => Err(NotFound { message: "Not found _3".to_owned(), found_type: FoundType::News }),
        }
    }

    async fn update(&self, filter: Document, data: Document) -> Result<bool, bool> {
        let db = self.connection.database("project_management");
        let mut cursor = db.collection(self.collection.as_str()).update_one(filter, data, None).await;
        return match cursor {
            Ok(_) => {
                Ok(true)
            }
            Err(_) => {
                Err(false)
            }
        };
    }

    async fn delete_one(&self, filter: Document) -> Result<bool, bool> {
        let db = self.connection.database("project_management");
        let mut cursor = db.collection(self.collection.as_str()).delete_one(filter, None).await;
        match cursor {
            Ok(_) => {
                Ok(true)
            }
            Err(_) => {
                Err(false)
            }
        }
    }

    async fn get_by_custom_query<T>(&self, filter: Document) -> Vec<T> where T: DeserializeOwned + 'static + Sized + Send {
        let db = self.connection.database("project_management");
        let mut cursor = db.collection(self.collection.as_str()).find(filter, None).await.unwrap();
        let mut data: Vec<T> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    match from_bson::<T>(Bson::Document(document)) {
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
}
