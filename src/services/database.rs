use std::env;
use std::str::FromStr;

use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, from_document, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::user_model::User;
use crate::models::{booking_model::Booking, car_model::Car, owner_model::Owner};

const DEFAULT_MONGODB_DSN: &str = "mongodb://localhost:27017/test?directConnection=true";

pub struct Database {
    booking: Collection<Booking>,
    owner: Collection<Owner>,
    car: Collection<Car>,

    user: Collection<User>,
}

impl Database {
    pub async fn new() -> Self {
        let uri = match env::var("MONGODB_DSN") {
            Ok(uri) => uri.to_string(),
            Err(_) => DEFAULT_MONGODB_DSN.to_string(),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let database = client.database("test");

        let booking: Collection<Booking> = database.collection("booking");
        let owner: Collection<Owner> = database.collection("owner");
        let car: Collection<Car> = database.collection("car");

        // Auth
        let user: Collection<User> = database.collection("user");

        Self {
            booking,
            owner,
            car,
            user,
        }
    }

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let result = self
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("Error creating owner");

        Ok(result)
    }

    pub async fn create_car(&self, car: Car) -> Result<InsertOneResult, Error> {
        let result = self
            .car
            .insert_one(car)
            .await
            .ok()
            .expect("Error creating car");

        Ok(result)
    }

    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let result = self
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("Error creating booking");

        Ok(result)
    }

    pub async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .booking
            .update_one(
                doc! {
                    "_id": ObjectId::from_str(booking_id).expect("Failed to parse booking_id")
                },
                doc! {
                    "$set": doc! {
                        "cancelled": true
                    }
                },
            )
            .await
            .ok()
            .expect("Error cancelling booking");

        Ok(result)
    }

    pub async fn register_user(&self, user: User) -> Result<InsertOneResult, Error> {
        let result = self
            .user
            .insert_one(user)
            .await
            .ok()
            .expect("Error creating user");

        Ok(result)
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let mut results = self
            .user
            .aggregate(
                vec![
                    doc! {
                        "$match": {
                            "active": true
                        }
                    }
                ]
            )
            .await
            .ok()
            .expect("Error getting users");

        let mut users: Vec<User> = Vec::new();

        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let user: User = from_document(doc).expect("Error converting document to User");
                    users.push(user);
                }
                Err(err) => panic!("Error getting users: {}", err),
            }
        }

        Ok(users)
    }
}
