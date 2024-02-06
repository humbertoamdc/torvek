use crate::app_state::AppState;
use crate::orders::usecases::create_order::AdminCreateOrderUseCase;
use crate::shared::usecase::UseCase;
use api_boundary::orders::requests::AdminCreateOrderRequest;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn admin_create_order(
    State(app_state): State<AppState>,
    Json(request): Json<AdminCreateOrderRequest>,
) -> impl IntoResponse {
    let usecase = AdminCreateOrderUseCase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
