use std::io::Result;
use actix_web::{HttpServer, App, get, HttpResponse, web};
use futures::executor::block_on;
use std::time::Duration;
use background_jobs::email_worker::email_worker::{EmailWorker, TEmailWorker};
use cache::redis::redis::Redis;
use std::sync::{Mutex, Arc};
use actix_cors::Cors;

// Mod
mod user;
mod board;

// Email Worker
async fn email_worker_process() {
    let worker = EmailWorker {};
    loop {
        match worker.reserve().await {
            Ok(_) => {
                println!("Job reserved..");
            }
            Err(_) => {
                println!("Any job not found.");
            }
        };
        println!("Checking jobs..");
        std::thread::sleep(Duration::from_millis(3000));
    }
}

#[actix_rt::main]
async fn main() -> Result<()> {
    // Create Email Worker
    let worker = EmailWorker {};
    let email_worker = web::Data::new(Arc::new(Mutex::new(worker)));

    // Create Redis Pool (Mutex)
    let redis_pool = web::Data::new(Mutex::new(Redis {}));

    std::thread::spawn(|| {
        let worker_fn = email_worker_process();
        block_on(worker_fn);
    });

    HttpServer::new(move || {
        App::new()
            // Boards
            .service(web::scope("/user").configure(user::routes::init_routes))
            .service(web::scope("/board").configure(board::routes::init_routes))
            // Pass App Data
            .app_data(email_worker.clone())
            // Pass Redis Pool
            .app_data(redis_pool.clone())
            .wrap(Cors::new().supports_credentials().finish())
    })
        .keep_alive(Some(75))
        .bind("192.168.5.111:5004").unwrap()
        .run()
        .await
}
