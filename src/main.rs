use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Zurl — URL Shortener")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Zurl running at http://127.0.0.1:8080");

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
