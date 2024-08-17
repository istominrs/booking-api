use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub brand: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarRequest {
    pub owner: String,
    pub brand: String,
    pub model: String,
}

impl TryFrom<CarRequest> for Car {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: CarRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&value.owner).expect("Failed to parse owner"),
            brand: value.brand,
            model: value.model,
        })
    }
}
