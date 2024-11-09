use crate::common::api_error::ApiError;
use crate::common::error::Error::*;
use crate::common::into_error_response::IntoError;
use axum::Json;
use http::StatusCode;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the request item doesn't exist")]
    ItemNotFoundError,
    #[error("no quote selected for part with id `{0}`")]
    NoSelectedQuoteAvailableForPart(String),
    #[error("cannot update parts after paying the quotation")]
    UpdatePartAfterPayingQuotation,
    #[error("invalid url couldn't be parsed")]
    InvalidUrl,
    #[error("cannot delete a project contains payed quotes")]
    DeleteLockedProject,
    #[error("cannot delete a quote that has been payed for")]
    DeletePayedQuotation,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl IntoError for Error {
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
            DeleteLockedProject => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: DeleteLockedProject.to_string(),
                },
            ),
            DeletePayedQuotation => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: DeletePayedQuotation.to_string(),
                },
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ApiError::default()),
        };

        (status_code, Json(api_error.into()))
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    #[default]
    UnknownError,
    ItemNotFound,
    MissingUserInput,
    NotAllowed,
    BadInput,
}
