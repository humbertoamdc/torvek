use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use api_boundary::orders::requests::QueryOrdersByStatusRequest;

use crate::app_state::AppState;
use crate::orders::usecases::query_orders_by_status::QueryOrdersByStatusUseCase;
use crate::shared::usecase::UseCase;

pub async fn query_orders_by_status(
    State(app_state): State<AppState>,
    Query(request): Query<QueryOrdersByStatusRequest>,
) -> impl IntoResponse {
    let usecase = QueryOrdersByStatusUseCase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
