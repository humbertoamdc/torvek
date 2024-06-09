use crate::common::api_error::ApiError;
use axum::Json;
use http::StatusCode;

pub trait IntoErrorResponse {
    fn into_error_response(self) -> (StatusCode, Json<ApiError>);
}
