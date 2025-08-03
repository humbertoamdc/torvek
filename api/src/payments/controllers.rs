use crate::app_state::AppState;
use crate::payments::models::inputs::{
    CompleteCheckoutSessionWebhookRequest, CreateCheckoutSessionInput,
};
use crate::payments::usecases::create_checkout_session::CreateCheckoutSession;
use crate::payments::usecases::create_orders_and_confirm_quotation_payment::CreateOrdersAndConfirmQuotationPayment;
use crate::shared::extractors::session::CustomerSession;
use crate::shared::extractors::stripe_event::StripeEvent;
use crate::shared::into_error_response::IntoError;
use crate::shared::UseCase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use stripe::{EventObject, EventType};
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCheckoutSessionRequest {
    pub project_id: String,
    pub quotation_id: String,
}

pub async fn create_checkout_session(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<CreateCheckoutSessionRequest>,
) -> impl IntoResponse {
    let input = CreateCheckoutSessionInput {
        identity: session.identity,
        project_id: request.project_id,
        quotation_id: request.quotation_id,
    };
    let usecase = CreateCheckoutSession::new(
        app_state.payments.stripe_client,
        app_state.parts.dynamodb_parts,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn complete_checkout_session_webhook(
    State(app_state): State<AppState>,
    StripeEvent(event): StripeEvent,
) -> impl IntoResponse {
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                if let Ok(request) = CompleteCheckoutSessionWebhookRequest::try_from(session) {
                    let usecase = CreateOrdersAndConfirmQuotationPayment::new(
                        app_state.projects.dynamodb_projects,
                        app_state.quotes.dynamodb_quotes,
                        app_state.orders.dynamodb_orders,
                        app_state.parts.dynamodb_parts,
                        Arc::new(Mutex::new(app_state.payments.transaction)),
                        app_state.services.emailer.ses,
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
