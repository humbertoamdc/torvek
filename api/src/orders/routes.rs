use axum::routing::{get, patch};
use axum::Router;

use crate::app_state::AppState;
use crate::orders::controllers::{admin_update_order_payout, query_orders_by_status};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/orders", get(query_orders_by_status))
        .route("/admin/orders/payout", patch(admin_update_order_payout))
}
