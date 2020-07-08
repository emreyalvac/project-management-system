use std::io::Result;
use actix_web::{HttpServer, App, web};
use futures::executor::block_on;
use std::time::Duration;
use background_jobs::email_worker::email_worker::{EmailWorker, TEmailWorker};
use std::sync::{Arc, RwLock};
use actix_cors::Cors;
use data_access::database::database_connection::{DatabaseConnection, TDatabaseConnection};
use mongodb::Client;

// Mod
mod user;
mod board;

// Email Worker
async fn email_worker_process() {
    let worker = EmailWorker {};
    loop {
        match worker.reserve().await {
            Ok(_) => {
                // println!("Job reserved..");
            }
            Err(_) => {
                // println!("Any job not found.");
            }
        };
        // println!("Checking jobs..");
        std::thread::sleep(Duration::from_millis(3000));
    }
}

#[actix_rt::main]
async fn main() -> Result<()> {
    // Create Redis Pool (Mutex)

    // Create Email Worker
    let worker = EmailWorker {};
    let email_worker = web::Data::new(Arc::new(RwLock::new(worker)));

    // Database Connection Pool
    let database: DatabaseConnection = DatabaseConnection {};
    let connection = database.get_connection().await.ok().unwrap();
    // Mutex only one thread at the same time
    // RwLock many reader, only one writer
    let database_pool: web::Data<RwLock<Client>> = web::Data::new(RwLock::new(connection));

    std::thread::spawn(|| {
        let worker_fn = email_worker_process();
        block_on(worker_fn);
    });

    HttpServer::new(move || {
        App::new()
            // Boards
            .service(web::scope("/user").configure(user::routes::init_routes))
            .service(web::scope("/board").configure(board::routes::init_routes))
            // Database Pool
            .app_data(database_pool.clone())
            // Pass App Data
            .app_data(email_worker.clone())
            // Pass Redis Pool
            .wrap(Cors::new().supports_credentials().finish())
    })
        .keep_alive(Some(75))
        .bind("127.0.0.1:4000").unwrap()
        .run()
        .await
}
