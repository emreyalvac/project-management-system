use actix_web::{get, post, web, HttpResponse};
use services::user_services::user::{UserServices, TUserServices};
use domain::user::login_user::LoginUser;
use domain::user::register::Register;
use futures::executor::block_on;
use background_jobs::email_worker::email_worker::{EmailWorker, TEmailWorker, EmailJob};
use std::sync::{Mutex, Arc};
use domain::user::user_get_by_id::UserGetById;
use middlewares::auth::auth::AuthorizationService;
use domain::user::insert_board_to_user::InsertBoardToUser;

#[post("/login")]
async fn login(user: web::Json<LoginUser>) -> HttpResponse {
    let services = UserServices {};
    let result = services.login(user.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/register")]
async fn register(register: web::Json<Register>, email_job: web::Data<Arc<Mutex<EmailWorker>>>) -> HttpResponse {
    let into = register.into_inner();
    let cloned = into.clone();
    let services = UserServices {};
    let result = services.register(cloned).await;
    match result {
        Ok(res) => {
            // Email Validation
            let cloned = into.clone();
            std::thread::spawn(move || {
                let worker = email_job.lock().unwrap();
                let validate = block_on(services.generate_token_for_validation(into));
                block_on(worker.enqueue(EmailJob { to: cloned.email, message: format!("Validation Key -> {}", validate), subject: "Validation".to_owned(), iterate: 1, class: "EmailClass".to_owned() }));
            });
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[get("/validate/{token}")]
async fn validate_token(token: web::Path<String>) -> HttpResponse {
    let services = UserServices {};
    let result = services.validate_user(token.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/getBoards")]
async fn get_user_boards(user: web::Json<UserGetById>, _: AuthorizationService) -> HttpResponse {
    let services = UserServices {};
    let result = services.get_user_boards(user.into_inner()).await;
    match result {
        Ok(data) => {
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            HttpResponse::Ok().json(e)
        }
    }
}

#[post("/createBoard")]
async fn insert_board_to_user(board: web::Json<InsertBoardToUser>) -> HttpResponse {
    let services = UserServices {};
    let result = services.insert_board(board.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(validate_token);
    cfg.service(get_user_boards);
    cfg.service(insert_board_to_user);
}
