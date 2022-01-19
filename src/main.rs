// #[macro_use]
// extern crate juniper;
// use juniper::FieldResult;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// my modules and crates
mod config;
mod database;
use crate::database::util::convert_cursor_to_vec;
use crate::database::DatabaseContainer;
mod models;
use crate::models::Status;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Hello world".to_string(),
    })
}

#[get("/users")]
async fn users(app_data: web::Data<AppState>) -> impl Responder {
    match convert_cursor_to_vec(
        app_data
            .get_ref()
            .database_container
            .users
            .find(None, None)
            .await,
    )
    .await
    {
        Ok(users_vec) => HttpResponse::Ok().json(users_vec),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/data_packets")]
async fn data_packets(app_data: web::Data<AppState>) -> impl Responder {
    match convert_cursor_to_vec(
        app_data
            .get_ref()
            .database_container
            .data_packets
            .find(None, None)
            .await,
    )
    .await
    {
        Ok(data_packets_vec) => HttpResponse::Ok().json(data_packets_vec),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    database_container: DatabaseContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    let config = crate::config::Config::build_config().unwrap();
    let data = AppState {
        database_container: DatabaseContainer::new(&config).await,
    };

    println!("Loaded database");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(hello)
            .service(users)
            .service(data_packets)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run();

    println!(
        "Running server at http://{}:{}",
        config.server.host, config.server.port
    );

    server.await
}
