use crate::quotations::errors::QuotationsError::*;
use crate::{ApiError, ErrorCode};
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum QuotationsError {
    #[error("error while creating quotation")]
    CreateQuotationError,
    #[error("error while querying quotations")]
    QueryQuotationsError,
    #[error("the quotation doesn't exist")]
    GetQuotationItemNotFoundError,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl Into<ApiError> for QuotationsError {
    fn into(self) -> ApiError {
        match self {
            CreateQuotationError => ApiError::default(),
            QueryQuotationsError => ApiError::default(),
            GetQuotationItemNotFoundError => ApiError {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                code: ErrorCode::ItemNotFound,
                message: GetQuotationItemNotFoundError.to_string(),
            },
            UnknownError => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                code: ErrorCode::UnknownError,
                message: String::from("an unexpected error occurred"),
            },
        }
    }
}
