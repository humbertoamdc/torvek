use api_boundary::common::into_error_response::IntoError;
use api_boundary::orders::requests::AdminUpdateOrderPayoutRequest;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_derive::Deserialize;

use crate::app_state::AppState;
use crate::orders::models::inputs::QueryOrdersForCustomerInput;
use crate::orders::usecases::admin_update_order_payout::AdminUpdateOrderPayoutUsecase;
use crate::orders::usecases::query_orders_for_customer::QueryOrdersForCustomer;
use crate::shared::extractors::session::{AdminSession, CustomerSession};
use crate::shared::UseCase;

#[derive(Deserialize)]
pub struct QueryOrdersForCustomerQueryParameters {
    pub with_part_data: Option<bool>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

pub async fn query_orders_for_customer(
    State(app_state): State<AppState>,
    Query(params): Query<QueryOrdersForCustomerQueryParameters>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = QueryOrdersForCustomerInput {
        identity: session.identity,
        with_part_data: params.with_part_data.unwrap_or(false),
        cursor: params.cursor,
        limit: params.limit.unwrap_or(10),
    };
    let usecase = QueryOrdersForCustomer::new(
        app_state.orders.orders_repository,
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_update_order_payout(
    State(app_state): State<AppState>,
    AdminSession(_): AdminSession,
    Json(request): Json<AdminUpdateOrderPayoutRequest>,
) -> impl IntoResponse {
    let usecase = AdminUpdateOrderPayoutUsecase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}
