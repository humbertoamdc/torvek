use crate::common::api_error::{ApiError, ErrorCode};
use crate::quotations::errors::QuotationsError::*;
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum QuotationsError {
    #[error("the quotation doesn't exist")]
    GetQuotationItemNotFoundError,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl Into<ApiError> for QuotationsError {
    fn into(self) -> ApiError {
        match self {
            GetQuotationItemNotFoundError => ApiError {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                code: ErrorCode::ItemNotFound,
                message: GetQuotationItemNotFoundError.to_string(),
            },
            UnknownError => ApiError::default(),
        }
    }
}
