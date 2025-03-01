use super::handlers;
use actix_web::web;
/// Configures the authentication route services for the app.
///
/// Registers the route services for both the register and login handlers
/// at the `/auth` path.
pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/auth").service(handlers::note_handler::register));
}
