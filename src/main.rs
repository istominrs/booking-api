mod models;
mod routes;
mod services;

use crate::routes::user_route::{get_users, register_user};
use crate::routes::{
    booking_route::{cancel_booking, create_booking},
    car_route::create_car,
    owner_route::create_owner,
};
use crate::services::database::Database;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
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
            .service(register_user)
            .service(get_users)
    })
    .bind(("localhost", 3030))?
    .run()
    .await
}
