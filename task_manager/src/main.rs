use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use main_error::MainError;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;
mod main_error;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), MainError> {
    // Removed the redundant namespace for MainError
    println!("Hello, world!");
    env_logger::init();

    let (_address, _port, _db) = (
        utils::constants::address.clone(),
        utils::constants::port.clone(),
        utils::constants::db_url.clone(),
    );

    let db: DatabaseConnection = Database::connect(_db).await.map_err(|_| MainError {
        message: "Database connection error".to_string(),
    })?;
    Migrator::up(&db, None).await.map_err(|e| MainError {
        message: format!("Database migration error: {}", e),
    })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .configure(routes::task_manager::config) // Fixed the reference to the config function
            .wrap(Logger::default())
    })
    .bind((_address, _port))
    .map_err(|_| MainError {
        // Removed redundant namespace for MainError
        message: "Server binding error".to_string(),
    })?
    .run()
    .await
    .map_err(|_| main_error::MainError {
        message: "Server run error".to_string(),
    })
}
