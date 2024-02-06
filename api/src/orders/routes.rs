use axum::routing::post;
use axum::Router;

use crate::app_state::AppState;
use crate::orders::controllers::create_order;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/orders", post(create_order))
}
