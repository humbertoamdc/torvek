use crate::shared::api_error::ApiError;
use crate::shared::into_error_response::IntoError;
use axum::Json;
use http::StatusCode;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("email is already in use")]
    EmailTakenRegistrationError,
    #[error("invalid credentials")]
    InvalidCredentialsLoginError,
    #[error("password has been found in data breaches")]
    BreachedPasswordRegistrationError,
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
    #[error("cannot delete a quote that has been paid for")]
    DeletePayedQuotation,
    #[error("a pdf quote can't be generated because parts haven't been quoted yet")]
    NoPdfQuoteAvailable,
    #[error("`{0}` is required")]
    MissingRequiredParameter(String),
    #[error("operation forbidden")]
    Forbidden,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl IntoError for Error {
    fn into_error_response(self) -> (StatusCode, Json<ApiError>) {
        let (status_code, api_error) = match self {
            Error::EmailTakenRegistrationError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::EmailTakenRegistrationError.to_string(),
                },
            ),
            Error::InvalidCredentialsLoginError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::InvalidCredentialsLoginError.to_string(),
                },
            ),
            Error::BreachedPasswordRegistrationError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::BreachedPasswordRegistrationError.to_string(),
                },
            ),
            Error::ItemNotFoundError => (
                StatusCode::NOT_FOUND,
                ApiError {
                    status_code: StatusCode::NOT_FOUND.as_u16(),
                    code: ErrorCode::ItemNotFound,
                    message: Error::ItemNotFoundError.to_string(),
                },
            ),
            Error::NoSelectedQuoteAvailableForPart(message) => (
                StatusCode::BAD_REQUEST,
                ApiError {
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    code: ErrorCode::MissingUserInput,
                    message,
                },
            ),
            Error::UpdatePartAfterPayingQuotation => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::UpdatePartAfterPayingQuotation.to_string(),
                },
            ),
            Error::InvalidUrl => (
                StatusCode::BAD_REQUEST,
                ApiError {
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    code: ErrorCode::BadInput,
                    message: Error::InvalidUrl.to_string(),
                },
            ),
            Error::DeleteLockedProject => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::DeleteLockedProject.to_string(),
                },
            ),
            Error::DeletePayedQuotation => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::DeletePayedQuotation.to_string(),
                },
            ),
            Error::NoPdfQuoteAvailable => (
                StatusCode::BAD_REQUEST,
                ApiError {
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    code: ErrorCode::ItemNotFound,
                    message: Error::NoPdfQuoteAvailable.to_string(),
                },
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ApiError::default()),
        };

        (status_code, Json(api_error))
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
