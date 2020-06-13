use actix_web::{get, post, web, HttpResponse};
use services::board_services::board::{BoardServices, TBoardServices};
use domain::board::board_get_with_id::BoardGetWithId;
use middlewares::auth::auth::AuthorizationService;
use domain::card::insert_card::InsertCard;
use services::card_services::card::{CardServices, TCardServices};

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
async fn board_create_card(card: web::Json<InsertCard>) -> HttpResponse {
    let services = CardServices {};
    let result = services.insert_card(card.into_inner()).await;
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
}
