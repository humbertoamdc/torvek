use crate::app_state::AppState;
use crate::payments::controllers::{create_checkout_session, handle_webhook};
use axum::routing::post;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route(
            "/payments/create_checkout_session",
            post(create_checkout_session),
        )
        .route("/payments/stripe/webhooks", post(handle_webhook))
}
