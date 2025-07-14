use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde_derive::Deserialize;

use crate::app_state::AppState;
use crate::orders::models::inputs::QueryOrdersForCustomerInput;
use crate::orders::usecases::query_orders_by_customer::QueryOrdersByCustomer;
use crate::shared::extractors::session::CustomerSession;
use crate::shared::into_error_response::IntoError;
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
    let usecase = QueryOrdersByCustomer::new(
        app_state.orders.dynamodb_orders,
        app_state.parts.dynamodb_parts,
        app_state.parts.s3,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(err.into_error_response()),
    }
}
