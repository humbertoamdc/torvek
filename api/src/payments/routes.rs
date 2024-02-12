use axum::routing::post;
use axum::Router;

use crate::app_state::AppState;
use crate::payments::controllers::{complete_checkout_session_webhook, create_checkout_session};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route(
            "/payments/create_checkout_session",
            post(create_checkout_session),
        )
        .route(
            "/payments/webhooks/complete_checkout_session",
            post(complete_checkout_session_webhook),
        )
}
