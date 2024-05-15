use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use stripe::{EventObject, EventType};

use api_boundary::payments::requests::{
    CompleteCheckoutSessionWebhookRequest, CreateCheckoutSessionRequest,
};

use crate::app_state::AppState;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::payments::usecases::create_checkout_session::CreateCheckoutSessionUseCase;
use crate::payments::usecases::create_orders_and_confirm_quotation_payment::CreateOrdersAndConfirmQuotationPaymentUseCase;
use crate::shared::extractors::stripe_event::StripeEvent;
use crate::shared::usecase::UseCase;

pub async fn create_checkout_session(
    State(app_state): State<AppState>,
    Json(request): Json<CreateCheckoutSessionRequest>,
) -> impl IntoResponse {
    let query_parts_for_quotation_usecase = QueryPartsForQuotationUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let usecase = CreateCheckoutSessionUseCase::new(
        app_state.payments.payments_processor,
        query_parts_for_quotation_usecase,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn complete_checkout_session_webhook(
    State(app_state): State<AppState>,
    StripeEvent(event): StripeEvent,
) -> impl IntoResponse {
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                if let Ok(request) =
                    CompleteCheckoutSessionWebhookRequest::try_from(session.metadata)
                {
                    let query_parts_for_quotation_usecase = QueryPartsForQuotationUseCase::new(
                        app_state.parts.parts_repository,
                        app_state.parts.object_storage,
                    );
                    let usecase = CreateOrdersAndConfirmQuotationPaymentUseCase::new(
                        app_state.payments.orders_creation_service,
                        query_parts_for_quotation_usecase,
                    );

                    let result = usecase.execute(request).await;

                    match result {
                        Ok(_) => Ok(StatusCode::OK),
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
