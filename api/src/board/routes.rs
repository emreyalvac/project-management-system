use actix_web::{get, post, web, HttpResponse, HttpRequest};
use services::board_services::board::{BoardServices, TBoardServices};
use domain::board::board_get_with_id::BoardGetWithId;
use middlewares::auth::auth::AuthorizationService;
use services::card_services::card::{CardServices, TCardServices};
use domain::board::insert_card_to_board::InsertCardToBoard;
use domain::card::insert_task_to_card::InsertTask;
use domain::card::move_task_to_another_card::MoveTaskToAnotherCard;
use domain::card::update_card::UpdateCard;
use domain::task::update_task::UpdateTask;
use services::task_services::task::{TaskServices, TTaskServices};
use domain::task::delete_task::DeleteTask;
use domain::task::assign_task_to_user::AssignTaskToUser;
use domain::task::delete_assigned_user_from_task::DeleteAssignedUserFromTask;
use std::sync::RwLock;
use mongodb::Client;

#[get("/{id}")]
async fn get_boards(id: web::Path<String>, _: AuthorizationService, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = BoardServices { client };
    let result = services.get_board_as_aggregate(BoardGetWithId { board_id: id.into_inner() }).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().json(err)
    }
}

#[post("/createCard")]
async fn board_create_card(card: web::Json<InsertCardToBoard>, _: AuthorizationService, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let board_services = BoardServices { client };
    let result = board_services.insert_card_to_board(card.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/card/createTask")]
async fn insert_task_to_card(task: web::Json<InsertTask>, _: AuthorizationService, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = CardServices { client };
    let result = services.insert_task_to_card(task.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/card/moveTaskToAnotherCard")]
async fn move_task_to_another_card(task: web::Json<MoveTaskToAnotherCard>, _: AuthorizationService, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = CardServices { client };
    let result = services.move_task_to_another_card(task.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/getBoardUsers")]
async fn get_board_users(_: AuthorizationService, board: web::Json<BoardGetWithId>, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = BoardServices { client };
    let result = services.get_board_users(board.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/card/updateCard")]
async fn update_card(_: AuthorizationService, card: web::Json<UpdateCard>, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = CardServices { client };
    let result = services.update_card(card.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/task/updateTask")]
async fn update_task(_: AuthorizationService, task: web::Json<UpdateTask>, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = TaskServices { client };
    let result = services.update_task(task.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/task/deleteTask")]
async fn delete_task(_: AuthorizationService, task: web::Json<DeleteTask>, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = TaskServices { client };
    let result = services.delete_task(task.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/task/assignTaskToUser")]
async fn assign_task_to_user(_: AuthorizationService, task: web::Json<AssignTaskToUser>, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = TaskServices { client };
    let result = services.assign_task_to_user(task.into_inner()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::Ok().json(err)
        }
    }
}

#[post("/task/deleteAssignedUserFromTask")]
async fn delete_assigned_user_from_task(_: AuthorizationService, task: web::Json<DeleteAssignedUserFromTask>, database: web::Data<RwLock<Client>>) -> HttpResponse {
    // Database
    let lock_database = database.read().unwrap();
    let client = lock_database.to_owned();
    let services = TaskServices { client };
    let result = services.delete_assigned_user_from_task(task.into_inner()).await;
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
    cfg.service(get_boards);
    cfg.service(board_create_card);
    cfg.service(insert_task_to_card);
    cfg.service(move_task_to_another_card);
    cfg.service(get_board_users);
    cfg.service(update_card);
    cfg.service(update_task);
    cfg.service(delete_task);
    cfg.service(assign_task_to_user);
    cfg.service(delete_assigned_user_from_task);
}
