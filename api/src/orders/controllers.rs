use crate::app_state::AppState;
use crate::orders::usecases::create_order::CreateOrderUseCase;
use crate::shared::usecase::UseCase;
use api_boundary::orders::requests::CreateOrderRequest;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn create_order(
    State(app_state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> impl IntoResponse {
    let usecase = CreateOrderUseCase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
