use crate::{api::AppState, models::UrlShortenerRequest};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/shortener")]
pub async fn shortener(
    state: web::Data<AppState>,
    data: web::Json<UrlShortenerRequest>,
) -> impl Responder {
    let _pool = &state.pool;
    HttpResponse::Ok().body(format!("Shortener: {:?}", data))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(shortener);
}
