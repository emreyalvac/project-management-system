use actix_web::{get, web, HttpResponse};
use services::board_services::board::{BoardServices, TBoardServices};

#[get("/")]
async fn get_all() -> HttpResponse {
    let service = BoardServices {};
    let result = service.get_all().await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        },
        Err(e) => {
            HttpResponse::Ok().json(e)
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
}
