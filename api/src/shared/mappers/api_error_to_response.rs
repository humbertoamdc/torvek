use api_boundary::common::api_error::ApiError;
use axum::Json;
use http::StatusCode;

pub fn api_error_to_response(err: ApiError) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::from_u16(err.status_code).unwrap(),
        Json(err.into()),
    )
}
