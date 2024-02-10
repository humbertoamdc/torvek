use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use api_boundary::orders::requests::AdminCreateOrdersRequest;

use crate::app_state::AppState;
use crate::orders::usecases::create_order::AdminCreateOrderUseCase;
use crate::shared::usecase::UseCase;

pub async fn admin_create_order(
    State(app_state): State<AppState>,
    Json(request): Json<AdminCreateOrdersRequest>,
) -> impl IntoResponse {
    let usecase = AdminCreateOrderUseCase::new(app_state.orders.orders_creation_service);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
