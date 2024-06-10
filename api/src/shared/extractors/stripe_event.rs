use axum::async_trait;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};
use http::{Request, StatusCode};
use stripe::{Event, Webhook};

use crate::app_state::AppState;

pub struct StripeEvent(pub Event);

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
    async fn from_request(
        req: Request<axum::body::Body>,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
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
