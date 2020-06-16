use actix_web::{get, post, web, HttpResponse, HttpRequest};
use services::board_services::board::{BoardServices, TBoardServices};
use domain::board::board_get_with_id::BoardGetWithId;
use middlewares::auth::auth::AuthorizationService;
use services::card_services::card::{CardServices, TCardServices};
use domain::board::insert_card_to_board::InsertCardToBoard;
use domain::card::insert_task_to_card::InsertTask;
use domain::card::move_task_to_another_card::MoveTaskToAnotherCard;

#[get("/{id}")]
async fn get_boards(id: web::Path<String>, _: AuthorizationService) -> HttpResponse {
    let services = BoardServices {};
    let result = services.get_board_as_aggregate(BoardGetWithId { board_id: id.into_inner() }).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().json(err)
    }
}

#[post("/createCard")]
async fn board_create_card(card: web::Json<InsertCardToBoard>, _: AuthorizationService) -> HttpResponse {
    let board_services = BoardServices {};
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
async fn insert_task_to_card(task: web::Json<InsertTask>, _: AuthorizationService) -> HttpResponse {
    let services = CardServices {};
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
async fn move_task_to_another_card(task: web::Json<MoveTaskToAnotherCard>, _: AuthorizationService) -> HttpResponse {
    let services = CardServices {};
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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_boards);
    cfg.service(board_create_card);
    cfg.service(insert_task_to_card);
    cfg.service(move_task_to_another_card);
}
