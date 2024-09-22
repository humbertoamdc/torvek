use crate::common::api_error::ApiError;
use crate::common::into_error_response::IntoErrorResponse;
use axum::Json;
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum PartsError {
    #[error("the part doesn't exist")]
    PartItemNotFound,
    #[error("no quote selected for part with id `{0}`")]
    NoSelectedQuoteAvailableForPart(String),
    #[error("invalid url couldn't be parsed")]
    InvalidUrl,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl IntoErrorResponse for PartsError {
    fn into_error_response(self) -> (StatusCode, Json<ApiError>) {
        let (status_code, api_error) = match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ApiError::default()),
        };

        (status_code, Json(api_error.into()))
    }
}
