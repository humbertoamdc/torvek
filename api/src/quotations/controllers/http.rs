use crate::app_state::AppState;
use crate::quotations::usecases::create_quotation::CreateQuotationUseCase;
use crate::quotations::usecases::query_quotations_for_project::QueryQuotationsForProjectUseCase;
use crate::quotations::usecases::UseCase;
use crate::shared::extractors::stripe_event::StripeEvent;
use api_boundary::quotations::requests::{
    CreateQuotationRequest, QueryQuotationsForProjectRequest,
};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn create_quotation(
    State(app_state): State<AppState>,
    Json(request): Json<CreateQuotationRequest>,
) -> impl IntoResponse {
    let usecase = CreateQuotationUseCase::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn query_quotations_for_project(
    State(app_state): State<AppState>,
    Path((client_id, project_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let usecase = QueryQuotationsForProjectUseCase::new(app_state.quotations.quotations_repository);
    let request = QueryQuotationsForProjectRequest::new(client_id, project_id);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn _pay_quotation_webhook(
    State(_app_state): State<AppState>,
    StripeEvent(_event): StripeEvent,
) -> impl IntoResponse {
    todo!("Implement me")
}
