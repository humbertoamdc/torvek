use serde::Deserialize;
use serde_derive::Serialize;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub role: UserRole,
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    Client,
}
