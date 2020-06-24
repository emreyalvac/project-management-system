use actix_web::{get, post, web, HttpResponse, HttpRequest};
use services::user_services::user::{UserServices, TUserServices};
use domain::user::login_user::LoginUser;
use domain::user::register::Register;
use futures::executor::block_on;
use background_jobs::email_worker::email_worker::{EmailWorker, TEmailWorker, EmailJob};
use std::sync::{Mutex, Arc};
use domain::user::user_get_by_id::UserGetById;
use middlewares::auth::auth::AuthorizationService;
use domain::user::insert_board_to_user::InsertBoardToUser;
use helpers::token_decoder::token_decoder;
use domain::common::claims::Claims;
use domain::user::invite_user_to_board::InviteUserToBoard;
use domain::user::get_by_email::GetByEmail;

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
async fn get_user_boards(_: AuthorizationService, req: HttpRequest) -> HttpResponse {
    let header = req.headers().get("Authorization").unwrap().to_str().unwrap().to_string();
    let result = token_decoder::<Claims>(header);
    let user = result.unwrap().sub;
    let services = UserServices {};
    let result = services.get_user_boards(UserGetById { user_id: user }).await;
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
async fn insert_board_to_user(_: AuthorizationService, mut board: web::Json<InsertBoardToUser>, req: HttpRequest) -> HttpResponse {
    let header = req.headers().get("Authorization").unwrap().to_str().unwrap().to_string();
    let result = token_decoder::<Claims>(header);
    let user = result.unwrap().sub;
    board.user_id = (&user).parse().unwrap();
    board.board_manager_user_id = user;
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

#[get("/getUserInformations")]
async fn get_user_informations(_: AuthorizationService, req: HttpRequest) -> HttpResponse {
    let header = req.headers().get("Authorization").unwrap().to_str().unwrap().to_string();
    let result = token_decoder::<Claims>(header);
    let user = result.unwrap().sub;
    let services = UserServices {};
    let result = services.get_by_id(UserGetById { user_id: user }).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::NotFound().json(err)
        }
    }
}

#[post("/inviteUserToBoard")]
async fn invite_user_to_board(_: AuthorizationService, email_job: web::Data<Arc<Mutex<EmailWorker>>>, invite: web::Json<InviteUserToBoard>) -> HttpResponse {
    let services = UserServices {};
    let inner_invite = invite.into_inner();
    match services.invite_user_with_email(inner_invite).await {
        Ok(response) => {
            let worker = email_job.lock().unwrap();
            let email_result = block_on(worker.enqueue(EmailJob { to: response.email, message: format!("Invite Token -> {}", response.token), subject: "Invite User".to_owned(), iterate: 1, class: "EmailClass".to_owned() }));
            match email_result {
                Ok(_) => {
                    HttpResponse::Ok().json(true)
                }
                Err(_) => {
                    HttpResponse::Ok().json(false)
                }
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[get("/checkAndApplyInvite/{token}")]
async fn check_and_apply_invite(_: AuthorizationService, token: web::Path<String>) -> HttpResponse {
    let services = UserServices {};
    let result = services.check_and_apply_invite(token.into_inner()).await;
    match result {
        Ok(_) => {
            HttpResponse::Ok().json(true)
        }
        Err(_) => {
            HttpResponse::Ok().json(false)
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(validate_token);
    cfg.service(get_user_boards);
    cfg.service(insert_board_to_user);
    cfg.service(get_user_informations);
    cfg.service(invite_user_to_board);
    cfg.service(check_and_apply_invite);
}
