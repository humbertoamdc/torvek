use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::fmt::{Display, Formatter};

pub mod common;
pub mod orders;
pub mod parts;
pub mod payments;
pub mod projects;
pub mod quotations;

#[derive(thiserror::Error, Debug, Default, Deserialize, Serialize)]
pub struct ApiError {
    pub status_code: u16,
    pub code: ErrorCode,
    pub message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> Self {
        ApiError::default()
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    #[default]
    UnknownError,
    ItemNotFound,
}
