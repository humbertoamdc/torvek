use api_boundary::common::into_error_response::IntoError;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_derive::Deserialize;

use api_boundary::orders::requests::{
    AdminUpdateOrderPayoutRequest, QueryOpenOrdersRequest, QueryOrdersForCustomerRequest,
};

use crate::app_state::AppState;
use crate::orders::usecases::admin_update_order_payout::AdminUpdateOrderPayoutUsecase;
use crate::orders::usecases::query_open_orders::QueryOpenOrdersUseCase;
use crate::orders::usecases::query_orders_for_customer::QueryOrdersForCustomer;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::shared::usecase::UseCase;

#[derive(Deserialize)]
pub struct QueryParameters {
    pub with_part_data: Option<bool>,
}

pub async fn query_orders_for_customer(
    State(app_state): State<AppState>,
    Path(customer_id): Path<String>,
    Query(params): Query<QueryParameters>,
) -> impl IntoResponse {
    let request = QueryOrdersForCustomerRequest {
        customer_id,
        with_part_data: params.with_part_data.unwrap_or(false),
    };
    let query_parts_for_quotation_usecase = QueryPartsForQuotationUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let usecase = QueryOrdersForCustomer::new(
        app_state.orders.orders_repository,
        query_parts_for_quotation_usecase,
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
