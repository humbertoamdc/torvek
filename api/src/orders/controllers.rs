use crate::app_state::AppState;
use crate::orders::usecases::create_order::AdminCreateOrderUseCase;
use crate::quotations::usecases::update_quotation_status::UpdateQuotationStatusUseCase;
use crate::shared::usecase::UseCase;
use api_boundary::orders::requests::AdminCreateOrdersRequest;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn admin_create_order(
    State(app_state): State<AppState>,
    Json(request): Json<AdminCreateOrdersRequest>,
) -> impl IntoResponse {
    let update_quotation_status_usecase =
        UpdateQuotationStatusUseCase::new(app_state.quotations.quotations_repository);
    let usecase = AdminCreateOrderUseCase::new(
        app_state.orders.orders_repository,
        update_quotation_status_usecase,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
