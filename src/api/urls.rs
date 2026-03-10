use crate::models::UrlShortenerRequest;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/shortener")]
pub async fn shortener(data: web::Json<UrlShortenerRequest>) -> impl Responder {
    // web::Json<T> automatically deserializes JSON into our struct
    HttpResponse::Ok().body(format!("Shortener: {:?}", data))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(shortener);
}
