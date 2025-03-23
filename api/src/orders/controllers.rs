use api_boundary::common::into_error_response::IntoError;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use api_boundary::orders::requests::{
    AdminUpdateOrderPayoutRequest, QueryOpenOrdersRequest, QueryOrdersForCustomerQueryParameters,
    QueryOrdersForCustomerRequest,
};

use crate::app_state::AppState;
use crate::orders::usecases::admin_update_order_payout::AdminUpdateOrderPayoutUsecase;
use crate::orders::usecases::query_open_orders::QueryOpenOrdersUseCase;
use crate::orders::usecases::query_orders_for_customer::QueryOrdersForCustomer;
use crate::shared::UseCase;

pub async fn query_orders_for_customer(
    State(app_state): State<AppState>,
    Path(customer_id): Path<String>,
    Query(params): Query<QueryOrdersForCustomerQueryParameters>,
) -> impl IntoResponse {
    let request = QueryOrdersForCustomerRequest::new(customer_id, params);
    let usecase = QueryOrdersForCustomer::new(
        app_state.orders.orders_repository,
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_open_orders(
    State(app_state): State<AppState>,
    Query(request): Query<QueryOpenOrdersRequest>,
) -> impl IntoResponse {
    let usecase = QueryOpenOrdersUseCase::new(app_state.orders.orders_repository);
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
