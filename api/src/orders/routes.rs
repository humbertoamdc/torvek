use axum::routing::{get, patch};
use axum::Router;

use crate::app_state::AppState;
use crate::orders::controllers::{
    admin_update_order_payout, query_open_orders, query_orders_for_customer,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route(
            "/customers/:customer_id/orders",
            get(query_orders_for_customer),
        )
        .route("/admin/orders/open", get(query_open_orders))
        .route("/admin/orders/payout", patch(admin_update_order_payout))
}
