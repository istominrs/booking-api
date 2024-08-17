mod models;
mod services;
mod routes;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use crate::routes::booking_route::{cancel_booking, create_booking};
use crate::routes::car_route::create_car;
use crate::routes::owner_route::create_owner;
use crate::services::database::Database;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new().await;
    let database_data = Data::new(database);

    HttpServer::new(move || {
        App::new()
            .app_data(database_data.clone())
            .service(ping)
            .service(create_owner)
            .service(create_booking)
            .service(create_car)
            .service(cancel_booking)
    })
        .bind(("localhost", 3030))?
        .run()
        .await
}
