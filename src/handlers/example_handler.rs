use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!\n")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Salve salve!")
}