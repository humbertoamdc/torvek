use axum::routing::get;
use axum::Router;

use crate::app_state::AppState;
use crate::orders::controllers::query_orders_for_customer;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/orders", get(query_orders_for_customer))
}
