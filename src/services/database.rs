use std::env;
use std::str::FromStr;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    Client,
    Collection, results::{InsertOneResult, UpdateResult},
};

use crate::models::{booking_model::Booking, car_model::Car, owner_model::Owner};

const DEFAULT_MONGODB_DSN: &str = "mongodb://localhost:27017/default?directConnection=true";

pub struct Database {
    booking: Collection<Booking>,
    owner: Collection<Owner>,
    car: Collection<Car>,
}

impl Database {
    pub async fn new() -> Self {
        let uri = match env::var("MONGODB_DSN") {
            Ok(uri) => uri.to_string(),
            Err(_) => DEFAULT_MONGODB_DSN.to_string(),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let database = client.database("default");

        let booking: Collection<Booking> = database.collection("booking");
        let owner: Collection<Owner> = database.collection("owner");
        let car: Collection<Car> = database.collection("car");

        Self {
            booking,
            owner,
            car,
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
}
