use core::num;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::services::example_service::give_a_number;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!\n")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Salve salve!")
}

pub async fn get_number() -> impl Responder {
    let number = give_a_number();
    HttpResponse::Ok().json(number)
}