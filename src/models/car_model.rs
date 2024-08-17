use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub id: Uuid,
    pub owner: String,
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
            id: Uuid::new_v4(),
            owner: value.owner,
            brand: value.brand,
            model: value.model,
        })
    }
}
