use crate::common::api_error::{ApiError, ErrorCode};
use crate::common::into_error_response::IntoErrorResponse;
use crate::parts::errors::PartsError::*;
use axum::Json;
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum PartsError {
    #[error("the part doesn't exist")]
    PartItemNotFound,
    #[error("no quote selected for part with id `{0}`")]
    NoSelectedQuoteAvailableForPart(String),
    #[error("can not update parts after paying the quotation")]
    UpdatePartAfterPayingQuotation,
    #[error("invalid url couldn't be parsed")]
    InvalidUrl,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl IntoErrorResponse for PartsError {
    fn into_error_response(self) -> (StatusCode, Json<ApiError>) {
        let (status_code, api_error) = match self {
            NoSelectedQuoteAvailableForPart(message) => (
                StatusCode::BAD_REQUEST,
                ApiError {
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    code: ErrorCode::MissingUserInput,
                    message,
                },
            ),
            UpdatePartAfterPayingQuotation => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: UpdatePartAfterPayingQuotation.to_string(),
                },
            ),
            InvalidUrl => (
                StatusCode::BAD_REQUEST,
                ApiError {
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    code: ErrorCode::BadInput,
                    message: InvalidUrl.to_string(),
                },
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ApiError::default()),
        };

        (status_code, Json(api_error.into()))
    }
}
