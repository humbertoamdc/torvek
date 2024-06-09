use crate::common::api_error::ApiError;
use crate::common::into_error_response::IntoErrorResponse;
use crate::payments::errors::PaymentsError::UnknownError;
use axum::Json;
use http::StatusCode;

#[derive(Debug)]
pub enum WebhookRequestError {
    MissingMetadata,
    MissingField,
}

#[derive(thiserror::Error, Debug)]
pub enum PaymentsError {
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl IntoErrorResponse for PaymentsError {
    fn into_error_response(self) -> (StatusCode, Json<ApiError>) {
        let (status_code, api_error) = match self {
            UnknownError => (StatusCode::INTERNAL_SERVER_ERROR, ApiError::default()),
        };

        (status_code, Json(api_error.into()))
    }
}
