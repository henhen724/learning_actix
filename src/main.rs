use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("PONG!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut config = config::Config::new();

    config
        .merge(config::File::with_name("config.toml").required(false))
        .unwrap();
    config.merge(config::Environment::default()).unwrap();

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/ping", web::get().to(manual_hello))
    })
    .bind(format!(
        "127.0.0.1:{}",
        config.get::<String>("port").unwrap()
    ))?
    .run();
    server.await
}
