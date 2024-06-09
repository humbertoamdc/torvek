use crate::common::api_error::ApiError;
use crate::parts::errors::PartsError::UnknownError;

#[derive(thiserror::Error, Debug)]
pub enum PartsError {
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl Into<ApiError> for PartsError {
    fn into(self) -> ApiError {
        match self {
            UnknownError => ApiError::default(),
        }
    }
}
