use crate::common::api_error::{ApiError, ErrorCode};
use crate::projects::errors::ProjectsError::*;
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum ProjectsError {
    #[error("error while creating project")]
    CreateProjectError,
    #[error("error while querying projects")]
    QueryProjectsError,
    #[error("the project doesn't exist")]
    GetProjectItemNotFoundError,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl Into<ApiError> for ProjectsError {
    fn into(self) -> ApiError {
        match self {
            CreateProjectError => ApiError::default(),
            QueryProjectsError => ApiError::default(),
            GetProjectItemNotFoundError => ApiError {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                code: ErrorCode::ItemNotFound,
                message: GetProjectItemNotFoundError.to_string(),
            },
            UnknownError => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                code: ErrorCode::UnknownError,
                message: String::from("an unexpected error occurred"),
            },
        }
    }
}
