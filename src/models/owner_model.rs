use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String
}

impl TryFrom<OwnerRequest> for Owner {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: OwnerRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone: value.phone,
        })
    }
}
