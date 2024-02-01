use crate::app_state::AppState;
use crate::quotations::usecases::admin_query_quotations_by_status::AdminQueryQuotationsByStatusUseCase;
use crate::quotations::usecases::confirm_quotation_payment::ConfirmQuotationPaymentWebhookUseCase;
use crate::quotations::usecases::create_quotation::CreateQuotationUseCase;
use crate::quotations::usecases::query_quotations_for_project::QueryQuotationsForProjectUseCase;
use crate::shared::extractors::stripe_event::StripeEvent;
use crate::shared::usecase::UseCase;
use api_boundary::quotations::requests::{
    AdminQueryQuotationsByStatusRequest, ConfirmQuotationPaymentWebhookRequest,
    CreateQuotationRequest, QueryQuotationsForProjectRequest,
};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use stripe::{EventObject, EventType};

pub async fn create_quotation(
    State(app_state): State<AppState>,
    Json(request): Json<CreateQuotationRequest>,
) -> impl IntoResponse {
    let usecase = CreateQuotationUseCase::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn query_quotations_for_project(
    State(app_state): State<AppState>,
    Path((client_id, project_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let usecase = QueryQuotationsForProjectUseCase::new(app_state.quotations.quotations_repository);
    let request = QueryQuotationsForProjectRequest::new(client_id, project_id);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn admin_query_quotations_by_status(
    State(app_state): State<AppState>,
    Query(request): Query<AdminQueryQuotationsByStatusRequest>,
) -> impl IntoResponse {
    let usecase =
        AdminQueryQuotationsByStatusUseCase::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn confirm_quotation_payment_webhook(
    State(app_state): State<AppState>,
    StripeEvent(event): StripeEvent,
) -> impl IntoResponse {
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                if let Ok(request) =
                    ConfirmQuotationPaymentWebhookRequest::try_from(session.metadata)
                {
                    let usecase = ConfirmQuotationPaymentWebhookUseCase::new(
                        app_state.quotations.quotations_repository,
                    );
                    let result = usecase.execute(request).await;

                    match result {
                        Ok(_) => Ok(StatusCode::NO_CONTENT),
                        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            } else {
                Err(StatusCode::UNPROCESSABLE_ENTITY)
            }
        }
        _ => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
