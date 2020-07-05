use cache::redis::redis::{Redis, TRedis};
use async_trait::async_trait;
use redis::{Commands, Value};
use serde::{Serialize, Deserialize};
use messaging::email::email::{Email, TEmail};
use domain::email::email::EmailSend;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailJob {
    pub class: String,
    pub to: String,
    pub iterate: i32,
    pub message: String,
    pub subject: String,
}

#[async_trait]
pub trait TEmailWorker {
    async fn enqueue(&self, job: EmailJob) -> Result<bool, bool>;
    async fn reserve(&self) -> Result<bool, bool>;
    async fn proc(&self, json_job: String) -> Result<bool, bool>;
}

// TODO: Take redis from actix_web app_data
#[derive(Debug)]
pub struct EmailWorker {}

#[async_trait]
impl TEmailWorker for EmailWorker {
    async fn enqueue(&self, job: EmailJob) -> Result<bool, bool> {
        let redis = Redis {};
        let mut redis_connection = redis.get_connection("redis://127.0.0.1".to_owned()).await.unwrap();
        let json_job = serde_json::to_string(&job).unwrap();
        let result = redis_connection.rpush::<String, String, Value>("queue::email_queue".to_owned(), json_job);
        println!("Enqueued Job -> {:?}", job);
        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(false)
        }
    }

    async fn reserve(&self) -> Result<bool, bool> {
        let redis = Redis {};
        let mut redis_connection = redis.get_connection("redis://127.0.0.1".to_owned()).await.unwrap();
        // Response
        let _res = match redis_connection.lindex::<String, String>("queue::email_queue".to_owned(), 0) {
            Ok(response) => {
                return Ok(self.proc(response).await.unwrap());
            }
            Err(_) => return Err(false)
        };
    }

    async fn proc(&self, json_job: String) -> Result<bool, bool> {
        let redis = Redis {};
        let mut redis_connection = redis.get_connection("redis://127.0.0.1".to_owned()).await.unwrap();
        let email = Email {};
        let mut job = serde_json::from_str::<EmailJob>(json_job.as_str()).unwrap();
        let send = email.send_email(EmailSend { from: "rusttestemail12@gmail.com".to_owned(), to: (&job.to).parse().unwrap(), body: (&job.message).parse().unwrap(), subject: (&job.subject).parse().unwrap() }).await;
        match send {
            Ok(_) => {
                println!("Job Completed {:?} and deleted", json_job);
                redis_connection.lpop::<String, Value>("queue::email_queue".to_owned());
            }
            Err(_) => {
                if job.iterate == 0 {
                    match redis_connection.lpop::<String, Value>("queue::email_queue".to_owned()) {
                        Ok(_) => {
                            println!("Queue Job Failed::System Error");
                        }
                        Err(_) => {
                            println!("Re queue error");
                        }
                    }
                } else {
                    match redis_connection.lpop::<String, Value>("queue::email_queue".to_owned()) {
                        Ok(_) => {
                            job.iterate = job.iterate - 1;
                            match self.enqueue(job).await {
                                Ok(_) => {
                                    println!("Re queue job..");
                                }
                                Err(_) => {
                                    println!("Re queue error");
                                }
                            };
                        }
                        Err(_) => {
                            println!("Re iterate error");
                        }
                    }
                }
                println!("Job Error");
            }
        }
        Ok(true)
    }
}
