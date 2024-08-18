use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse};

use crate::models::user_model::{User, UserRequest};
use crate::services::database::Database;

#[get("/users")]
pub async fn get_users(database: Data<Database>) -> HttpResponse {
    match database.get_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/register")]
pub async fn register_user(database: Data<Database>, request: Json<UserRequest>) -> HttpResponse {
    match database
        .register_user(
            User::try_from(UserRequest {
                email: request.email.clone(),
                password: request.password.clone(),
            })
            .expect("Error converting UserRequest to User"),
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().body("User registered"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
