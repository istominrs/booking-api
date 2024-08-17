use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub end_time: String,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: BookingRequest) -> Result<Self, Self::Error> {
        let chrono_start_time: SystemTime = chrono::DateTime::parse_from_rfc3339(&value.start_time)
            .map_err(|err| format!("Failed to parse start_time: {}", err))?
            .with_timezone(&Utc)
            .into();

        let chrono_end_time: SystemTime = chrono::DateTime::parse_from_rfc3339(&value.end_time)
            .map_err(|err| format!("Failed to parse end_time: {}", err))?
            .with_timezone(&Utc)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&value.owner).expect("Failed to parse owner"),
            start_time: DateTime::from(chrono_start_time),
            end_time: DateTime::from(chrono_end_time),
            cancelled: false,
        })
    }
}
