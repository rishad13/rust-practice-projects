use actix_web::{middleware::Logger, App, HttpServer};
mod main_error;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), main_error::MainError> {
    println!("Hello, world!");
    let (_address, _port) = (
        utils::constants::address.clone(),
        utils::constants::port.clone(),
    );

    HttpServer::new(move || App::new().wrap(Logger::default()).service(hello))
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
