use crate::{
    api::AppState, domain::DomainError, models::UrlShortenerRequest, services::UrlService,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[post("/shortener")]
pub async fn shortener(
    state: web::Data<AppState>,
    data: web::Json<UrlShortenerRequest>,
) -> impl Responder {
    match UrlService::create_short_url(&state.pool, data.into_inner()).await {
        Ok(url_short) => HttpResponse::Ok().json(json!({
            "message": "URL shortened successfully",
            "url_short": format!("http://127.0.0.1:8080/{}", url_short)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[get("/{code}")]
pub async fn redirect(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();

    match UrlService::get_url(&state.pool, &code).await {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),
        Err(DomainError::NotFound) => {
            HttpResponse::NotFound().json(json!({"error": "Short URL not found"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(shortener);
    cfg.service(redirect);
}
