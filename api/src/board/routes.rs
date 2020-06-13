use actix_web::{get, web, HttpResponse};
use services::board_services::board::{BoardServices, TBoardServices};
use domain::board::board_get_with_id::BoardGetWithId;
use middlewares::auth::auth::AuthorizationService;

#[get("/{id}")]
async fn get_boards(id: web::Path<String>, _: AuthorizationService) -> HttpResponse {
    let services = BoardServices {};
    let result = services.get_board_as_aggregate(BoardGetWithId { board_id: id.into_inner() }).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().json(err)
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_boards);
}
