// models.rs

use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Serialize)]
pub struct UserDto {
    pub(crate) name: String,
    pub(crate) created_at: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub(crate) username: String,
}

impl UserDto {
    pub fn new(name: String) -> Self {
        let created_at = Utc::now().to_string();
        UserDto { name, created_at }
    }
}
