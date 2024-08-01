use actix_web::web;
use actix_web::web::route;
use crate::handlers::example_handler;
use crate::handlers::example_handler::manual_hello;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(example_handler::hello))
            .route("/echo", web::post().to(example_handler::echo))
            .route("/manual_hello", web::get().to(manual_hello))
    );
}