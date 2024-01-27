use crate::app_state::AppState;
use crate::payments::usecases::create_checkout_session::CreateCheckoutSessionUseCase;
use crate::quotations::usecases::UseCase;
use crate::shared::extractors::stripe_event::StripeEvent;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use stripe::{EventObject, EventType};

pub async fn create_checkout_session(
    State(app_state): State<AppState>,
    Json(request): Json<CreateCheckoutSessionRequest>,
) -> impl IntoResponse {
    let usecase = CreateCheckoutSessionUseCase::new(app_state.payments.payments_processor);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[axum::debug_handler]
pub async fn handle_webhook(
    State(_): State<AppState>,
    StripeEvent(event): StripeEvent,
) -> impl IntoResponse {
    log::info!("Receiving events from Stripe.");
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                println!(
                    "Received checkout session completed webhook with id: {:?}",
                    session.id
                )
            }
        }
        _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
    }
}
