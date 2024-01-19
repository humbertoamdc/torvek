use crate::app_state::AppState;
use crate::parts::usecases::admin_query_parts_by_status::AdminQueryPartsByStatusUseCase;
use crate::parts::usecases::admin_update_part::AdminUpdatePartUseCase;
use crate::parts::usecases::create_parts::CreatePartsUseCase;
use crate::parts::usecases::drawing_upload_url::CreateDrawingUploadUrlUseCase;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::parts::usecases::update_part::UpdatePartUseCase;
use crate::parts::usecases::UseCase;
use api_boundary::parts::requests::{
    AdminQueryPartsByStatusRequest, AdminUpdatePartRequest, CreateDrawingUploadUrlRequest,
    CreatePartsRequest, QueryPartsForQuotationRequest, UpdatePartRequest,
};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn create_parts(
    State(app_state): State<AppState>,
    Json(request): Json<CreatePartsRequest>,
) -> impl IntoResponse {
    let usecase = CreatePartsUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn query_parts_for_quotation(
    State(app_state): State<AppState>,
    Path((client_id, project_id, quotation_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let usecase = QueryPartsForQuotationUseCase::new(app_state.parts.parts_repository);
    let request = QueryPartsForQuotationRequest::new(client_id, project_id, quotation_id);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn update_part(
    State(app_state): State<AppState>,
    Json(request): Json<UpdatePartRequest>,
) -> impl IntoResponse {
    let usecase = UpdatePartUseCase::new(app_state.parts.parts_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn create_drawing_upload_url(
    State(app_state): State<AppState>,
    Json(request): Json<CreateDrawingUploadUrlRequest>,
) -> impl IntoResponse {
    let usecase = CreateDrawingUploadUrlUseCase::new(app_state.parts.object_storage);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn admin_query_parts_by_status(
    State(app_state): State<AppState>,
    Query(request): Query<AdminQueryPartsByStatusRequest>,
) -> impl IntoResponse {
    let usecase = AdminQueryPartsByStatusUseCase::new(app_state.parts.parts_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn admin_update_part(
    State(app_state): State<AppState>,
    Json(request): Json<AdminUpdatePartRequest>,
) -> impl IntoResponse {
    let usecase = AdminUpdatePartUseCase::new(app_state.parts.parts_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
