use crate::common::api_error::ApiError;
use crate::orders::errors::OrdersError::UnknownError;

#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl Into<ApiError> for OrdersError {
    fn into(self) -> ApiError {
        match self {
            UnknownError => ApiError::default(),
        }
    }
}
