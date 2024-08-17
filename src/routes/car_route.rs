use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};

use crate::models::car_model::{Car, CarRequest};
use crate::services::database::Database;

#[post("/car")]
pub async fn create_car(database: Data<Database>, request: Json<CarRequest>) -> HttpResponse {
    match database
        .create_car(
            Car::try_from(CarRequest {
                owner: request.owner.clone(),
                brand: request.brand.clone(),
                model: request.model.clone(),
            }).expect("Error converting CarRequest to Car")
        )
        .await
    {
        Ok(car) => HttpResponse::Ok().json(car),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}