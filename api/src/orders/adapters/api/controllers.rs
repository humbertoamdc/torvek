use api_boundary::orders::requests::CreateDrawingUploadUrlRequest;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use crate::app_state::AppState;
use crate::orders::adapters::api::requests::{
    AdminQueryOrdersByStatusRequest, AdminUpdateOrderRequest, CreateOrdersRequest,
    QueryOrdersForClientRequest, UpdateOrderRequest,
};
use crate::orders::application::usecases::admin_query_orders_by_status::AdminQueryOrdersByStatusUseCase;
use crate::orders::application::usecases::admin_update_order::AdminUpdateOrderUseCase;
use crate::orders::application::usecases::create_orders::CreateOrdersUseCase;
use crate::orders::application::usecases::drawing_upload_url::CreateDrawingUploadUrlUseCase;
use crate::orders::application::usecases::interfaces::UseCase;
use crate::orders::application::usecases::query_orders_for_client::QueryOrdersForClientUseCase;
use crate::orders::application::usecases::update_order::UpdateOrderUseCase;

pub async fn query_orders_for_client(
    State(app_state): State<AppState>,
    Path(client_id): Path<String>,
) -> impl IntoResponse {
    let usecase = QueryOrdersForClientUseCase::new(app_state.orders.orders_repository);
    let request = QueryOrdersForClientRequest::new(client_id);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn create_orders_and_file_upload_urls(
    State(app_state): State<AppState>,
    Json(request): Json<CreateOrdersRequest>,
) -> impl IntoResponse {
    let usecase = CreateOrdersUseCase::new(
        app_state.orders.object_storage,
        app_state.orders.orders_repository,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn update_order(
    State(app_state): State<AppState>,
    Json(request): Json<UpdateOrderRequest>,
) -> impl IntoResponse {
    let usecase = UpdateOrderUseCase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn create_drawing_upload_url(
    State(app_state): State<AppState>,
    Json(request): Json<CreateDrawingUploadUrlRequest>,
) -> impl IntoResponse {
    let usecase = CreateDrawingUploadUrlUseCase::new(app_state.orders.object_storage);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn admin_query_orders_by_status(
    State(app_state): State<AppState>,
    Query(request): Query<AdminQueryOrdersByStatusRequest>,
) -> impl IntoResponse {
    let usecase = AdminQueryOrdersByStatusUseCase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn admin_update_order(
    State(app_state): State<AppState>,
    Json(request): Json<AdminUpdateOrderRequest>,
) -> impl IntoResponse {
    let usecase = AdminUpdateOrderUseCase::new(app_state.orders.orders_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
