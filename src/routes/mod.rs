use actix_web::web;

mod example_routes;

pub fn init(cfg: &mut web::ServiceConfig) {
    example_routes::init(cfg);
}