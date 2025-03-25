use super::handlers;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/task")
            .service(handlers::task_manager_handler::add_task)
            .service(handlers::task_manager_handler::get_task)
            .service(handlers::task_manager_handler::get_task_by_id),
    );
}
