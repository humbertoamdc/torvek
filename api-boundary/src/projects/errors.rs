use crate::common::api_error::{ApiError, ErrorCode};
use crate::projects::errors::ProjectsError::*;
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum ProjectsError {
    #[error("the project doesn't exist")]
    GetProjectItemNotFoundError,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl Into<ApiError> for ProjectsError {
    fn into(self) -> ApiError {
        match self {
            GetProjectItemNotFoundError => ApiError {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                code: ErrorCode::ItemNotFound,
                message: GetProjectItemNotFoundError.to_string(),
            },
            UnknownError => ApiError::default(),
        }
    }
}
