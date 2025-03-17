use actix_web::web;
use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/task")
            .service(handlers::task_manager_handler::add_task)
    );
}