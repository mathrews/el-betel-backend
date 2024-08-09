use actix_web::web;
use crate::handlers::example_handler;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(example_handler::hello))
            .route("/echo", web::post().to(example_handler::echo))
            .route("/manual_hello", web::get().to(example_handler::manual_hello))
            .route("/get-number", web::get().to(example_handler::get_number))
    );
}