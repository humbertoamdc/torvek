use crate::app_state::AppState;
use crate::parts::usecases::create_parts::CreatePartsUseCase;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::parts::usecases::UseCase;
use api_boundary::parts::requests::{CreatePartsRequest, QueryPartsForQuotationRequest};
use axum::extract::{Path, State};
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
