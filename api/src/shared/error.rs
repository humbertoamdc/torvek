use crate::shared::api_error::ApiError;
use crate::shared::into_error_response::IntoError;
use axum::Json;
use http::StatusCode;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Email is already in use")]
    EmailTakenRegistrationError,
    #[error("Invalid credentials")]
    InvalidCredentialsLoginError,
    #[error("Password has been found in data breaches")]
    BreachedPasswordRegistrationError,
    #[error("The request item doesn't exist")]
    ItemNotFoundError,
    #[error("No quote selected for part with id `{0}`")]
    NoSelectedQuoteAvailableForPart(String),
    #[error("Cannot update the quote after paying it")]
    QuoteIsInPayedStatus,
    #[error("Invalid url couldn't be parsed")]
    InvalidUrl,
    #[error("Cannot delete a project contains payed quotes")]
    DeleteLockedProject,
    #[error("Cannot delete a quote that has been paid for")]
    DeletePayedQuotation,
    #[error("A pdf quote can't be generated because parts haven't been quoted yet")]
    NoPdfQuoteAvailable,
    #[error("`{0}` is required")]
    MissingRequiredParameter(String),
    #[error("Operation forbidden")]
    Forbidden,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid part attributes: {0}")]
    InvalidPartAttributes(String),
    #[error("An unexpected error occurred")]
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
            Error::QuoteIsInPayedStatus => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiError {
                    status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::QuoteIsInPayedStatus.to_string(),
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
            Error::Forbidden => (
                StatusCode::FORBIDDEN,
                ApiError {
                    status_code: StatusCode::FORBIDDEN.as_u16(),
                    code: ErrorCode::NotAllowed,
                    message: Error::Forbidden.to_string(),
                },
            ),
            Error::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ApiError {
                    status_code: StatusCode::UNAUTHORIZED.as_u16(),
                    code: ErrorCode::Unauthorized,
                    message: Error::Unauthorized.to_string(),
                },
            ),
            Error::InvalidPartAttributes(message) => (
                StatusCode::BAD_REQUEST,
                ApiError {
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    code: ErrorCode::MissingUserInput,
                    message,
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
    Unauthorized,
    ItemNotFound,
    MissingUserInput,
    NotAllowed,
    BadInput,
}
