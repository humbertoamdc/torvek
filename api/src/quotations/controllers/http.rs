use crate::app_state::AppState;
use crate::quotations::usecases::create_quotation::CreateQuotationUseCase;
use crate::quotations::usecases::UseCase;
use api_boundary::quotations::requests::CreateQuotationRequest;
use axum::extract::State;
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
