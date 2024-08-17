use actix_web::{HttpResponse, post, put};
use actix_web::web::{Data, Json, Path};

use crate::models::booking_model::{Booking, BookingRequest};
use crate::services::database::Database;

#[post("/booking")]
pub async fn create_booking(database: Data<Database>, request: Json<BookingRequest>) -> HttpResponse {
    match database
        .create_booking(
            Booking::try_from(BookingRequest {
                owner: request.owner.clone(),
                start_time: request.start_time.clone(),
                end_time: request.end_time.clone(),
            }).expect("Error converting BookingRequest to Booking")
        )
        .await
    {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("booking/{id}/cancel")]
pub async fn cancel_booking(database: Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let booking_id = path.into_inner().0;

    match database.cancel_booking(booking_id.as_str()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}