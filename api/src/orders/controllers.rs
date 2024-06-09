use api_boundary::common::into_error_response::IntoErrorResponse;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use api_boundary::orders::requests::{AdminUpdateOrderPayoutRequest, QueryOrdersByStatusRequest};

use crate::app_state::AppState;
use crate::orders::usecases::admin_update_order_payout::AdminUpdateOrderPayoutUsecase;
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
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_update_order_payout(
    State(app_state): State<AppState>,
    Json(request): Json<AdminUpdateOrderPayoutRequest>,
) -> impl IntoResponse {
    let usecase = AdminUpdateOrderPayoutUsecase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}
