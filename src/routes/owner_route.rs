use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};

use crate::models::owner_model::{Owner, OwnerRequest};
use crate::services::database::Database;

#[post("/owner")]
pub async fn create_owner(database: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    match database
        .create_owner(
            Owner::try_from(OwnerRequest {
                first_name: request.first_name.clone(),
                last_name: request.last_name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                address: request.address.clone(),
            }).expect("Error converting OwnerRequest to Owner")
        )
        .await
    {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}