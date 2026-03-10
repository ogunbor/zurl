use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;
use std::env;

// Share the pool across handlers
pub struct AppState {
    pub db: sqlx::MySqlPool,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Zurl — URL Shortener")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Zurl running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
