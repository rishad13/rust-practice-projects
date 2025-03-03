use super::handlers;
use actix_web::web;
/// Configures the authentication route services for the app.
///
/// Registers the route services for both the register and login handlers
/// at the `/auth` path.
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/note")
            .service(handlers::note_handler::add_note)
            .service(handlers::note_handler::get_notes)
            .service(handlers::note_handler::get_all_notes),
    );
}
