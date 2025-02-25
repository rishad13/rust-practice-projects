use actix_web::{middleware::Logger, web, App, HttpServer};
use main_error::MainError;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;
mod main_error;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), main_error::MainError> {
    println!("Hello, world!");
    let (_address, _port, _db) = (
        utils::constants::address.clone(),
        utils::constants::port.clone(),
        utils::constants::db_url.clone(),
    );
    env_logger::init();

    let db: DatabaseConnection = Database::connect(_db).await.map_err(|_| MainError {
        message: "Database connection error".to_string(),
    })?;
    Migrator::up(&db, None).await.map_err(|e| MainError {
        message: format!("Database migration error: {}", e),
    })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .service(hello)
    })
    .bind((_address, _port))
    .map_err(|_| main_error::MainError {
        message: "Server binding error".to_string(),
    })?
    .run()
    .await
    .map_err(|_| main_error::MainError {
        message: "Server run error".to_string(),
    })
}

#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello world!")
}
