use crate::models::error::ErrorCode;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
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

impl Default for ApiError {
    fn default() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            code: ErrorCode::UnknownError,
            message: String::from("an unexpected error occurred"),
        }
    }
}
