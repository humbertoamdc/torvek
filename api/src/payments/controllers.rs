use crate::app_state::AppState;
use crate::payments::usecases::create_checkout_session::CreateCheckoutSessionUseCase;
use crate::quotations::usecases::UseCase;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use axum::extract::{FromRequest, State};
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{async_trait, Json};
use stripe::{Event, EventObject, EventType, Webhook};

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

pub struct StripeEvent(Event);

#[async_trait]
impl<B> FromRequest<AppState, B> for StripeEvent
where
    String: FromRequest<AppState, B>,
    B: Send + 'static,
{
    type Rejection = Response;

    // Since we are using the AppState directly, we need to call the State() extractor on the
    // handlers using this extractor before calling the StripeEvent extractor.
    /*
        fn handler(State(app_state): State(AppState), StripeEvent(event): StripeEvent) {
            ...
        }
    */
    // Notice that the order matters.
    async fn from_request(req: Request<B>, state: &AppState) -> Result<Self, Self::Rejection> {
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload = String::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self(
            Webhook::construct_event(
                &payload,
                signature.to_str().unwrap(),
                &state.payments.webhook_secret,
            )
            .map_err(|err| {
                log::error!("stripe webhook error: {err:?}");
                StatusCode::BAD_REQUEST.into_response()
            })?,
        ))
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
        EventType::AccountUpdated => {
            if let EventObject::Account(account) = event.data.object {
                println!(
                    "Received account updated webhook for account: {:?}",
                    account.id
                )
            }
        }
        _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
    }
}
