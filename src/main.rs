use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("PONG!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/ping", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:1234")?
    .run()
    .await
}