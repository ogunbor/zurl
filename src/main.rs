use actix_web::{web, App, HttpServer};
use zurl::{api, configuration, AppState, Settings};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let settings = Settings::from_env().expect("Failed to load configuration");

    // Create database pool
    let pool = configuration::database::create_pool(&settings.database_url)
        .await
        .expect("Failed to create database pool");

    // Create app state
    let app_state = web::Data::new(AppState { pool });

    println!(
        "🚀 Server starting at http://{}:{}",
        settings.host, settings.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // ← Share state with all workers
            .configure(api::urls::configure) // ← Configure routes
    })
    .bind((settings.host.as_str(), settings.port))?
    .run()
    .await
}
