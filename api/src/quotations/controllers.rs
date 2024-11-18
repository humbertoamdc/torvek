use api_boundary::common::into_error_response::IntoError;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::{
    AdminQueryQuotationsByStatusRequest, CreateQuotationRequest, DeleteQuotationRequest,
    GetQuotationByIdRequest, GetQuotationSubtotalRequest, QueryQuotationsForProjectRequest,
    SendQuotationForReviewRequest, UpdateQuotationStatusRequest,
};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use crate::app_state::AppState;
use crate::quotations::usecases::admin_query_quotations_by_status::AdminQueryQuotationsByStatusUseCase;
use crate::quotations::usecases::create_quotation::CreateQuotationUseCase;
use crate::quotations::usecases::delete_quotation::DeleteQuotationUseCase;
use crate::quotations::usecases::get_quotation_by_id::GetQuotationByIdUseCase;
use crate::quotations::usecases::get_quotation_subtotal::GetQuotationSubtotalUseCase;
use crate::quotations::usecases::query_quotations_for_project::QueryQuotationsForProjectUseCase;
use crate::quotations::usecases::update_quotation_status::UpdateQuotationStatusUseCase;
use crate::shared::UseCase;

pub async fn create_quotation(
    State(app_state): State<AppState>,
    Json(request): Json<CreateQuotationRequest>,
) -> impl IntoResponse {
    let usecase = CreateQuotationUseCase::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_quotations_for_project(
    State(app_state): State<AppState>,
    Path((customer_id, project_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let usecase = QueryQuotationsForProjectUseCase::new(app_state.quotations.quotations_repository);
    let request = QueryQuotationsForProjectRequest::new(customer_id, project_id);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_quotation_by_id(
    State(app_state): State<AppState>,
    Path(request): Path<GetQuotationByIdRequest>,
) -> impl IntoResponse {
    let usecase = GetQuotationByIdUseCase::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(quotation) => Ok((StatusCode::OK, Json(quotation))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_query_quotations_by_status(
    State(app_state): State<AppState>,
    Query(request): Query<AdminQueryQuotationsByStatusRequest>,
) -> impl IntoResponse {
    let usecase =
        AdminQueryQuotationsByStatusUseCase::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_quotation_subtotal(
    State(app_state): State<AppState>,
    Path(request): Path<GetQuotationSubtotalRequest>,
) -> impl IntoResponse {
    let usecase = GetQuotationSubtotalUseCase::new(
        app_state.parts.parts_repository,
        app_state.quotations.quotations_repository,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn delete_quotation(
    State(app_state): State<AppState>,
    Path(request): Path<DeleteQuotationRequest>,
) -> impl IntoResponse {
    let usecase = DeleteQuotationUseCase::new(
        app_state.quotations.quotations_repository,
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::NO_CONTENT, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn send_quotation_for_review(
    State(app_state): State<AppState>,
    Json(request): Json<SendQuotationForReviewRequest>,
) -> impl IntoResponse {
    let request = UpdateQuotationStatusRequest {
        project_id: request.project_id,
        quotation_id: request.quotation_id,
        status: QuotationStatus::PendingReview,
    };
    let usecase = UpdateQuotationStatusUseCase::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(quotation) => Ok((StatusCode::OK, Json(quotation))),
        Err(err) => Err(err.into_error_response()),
    }
}
