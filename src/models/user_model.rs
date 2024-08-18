use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::services::password::hash_password;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub password_hash: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

impl TryFrom<UserRequest> for User {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: UserRequest) -> Result<Self, Self::Error> {
        let password_hash = hash_password(&value.password)?;

        Ok(Self {
            _id: ObjectId::new(),
            email: value.email,
            password_hash,
            active: true,
        })
    }
}
