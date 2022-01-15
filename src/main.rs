// #[macro_use]
// extern crate juniper;
// use juniper::FieldResult;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// my modules and crates
mod config;
mod models;
use crate::models::Status;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users")]
async fn users() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = crate::config::Config::build_config().unwrap();

    let server = HttpServer::new(|| App::new().service(hello).service(users))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run();

    println!(
        "Running server at http://{}:{}",
        config.server.host, config.server.port
    );

    server.await
}
