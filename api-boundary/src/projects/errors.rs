use crate::common::api_error::{ApiError, ErrorCode};
use crate::common::into_error_response::IntoErrorResponse;
use crate::projects::errors::ProjectsError::*;
use axum::Json;
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum ProjectsError {
    #[error("the project doesn't exist")]
    GetProjectItemNotFoundError,
    #[error("an unexpected error occurred")]
    UnknownError,
}

impl IntoErrorResponse for ProjectsError {
    fn into_error_response(self) -> (StatusCode, Json<ApiError>) {
        let (status_code, api_error) = match self {
            GetProjectItemNotFoundError => (
                StatusCode::NOT_FOUND,
                ApiError {
                    status_code: StatusCode::NOT_FOUND.as_u16(),
                    code: ErrorCode::ItemNotFound,
                    message: GetProjectItemNotFoundError.to_string(),
                },
            ),
            UnknownError => (StatusCode::INTERNAL_SERVER_ERROR, ApiError::default()),
        };

        (status_code, Json(api_error.into()))
    }
}
