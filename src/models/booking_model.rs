use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub id: Uuid,
    pub owner: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: BookingRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            owner: value.owner,
            start_time: value.start_time,
            end_time: value.end_time,
            cancelled: false,
        })
    }
}
