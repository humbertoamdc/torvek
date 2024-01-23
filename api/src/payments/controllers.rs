use crate::app_state::AppState;
use crate::payments::usecases::create_checkout_session::CreateCheckoutSessionUseCase;
use crate::quotations::usecases::UseCase;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn create_checkout_session(
    State(app_state): State<AppState>,
    Json(request): Json<CreateCheckoutSessionRequest>,
) -> impl IntoResponse {
    let usecase = CreateCheckoutSessionUseCase::new(app_state.payments.payments_processor);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
