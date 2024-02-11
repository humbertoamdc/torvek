use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;
use crate::orders::controllers::{admin_create_order, query_orders_by_status};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/admin/orders", post(admin_create_order))
        .route("/orders", get(query_orders_by_status))
}
