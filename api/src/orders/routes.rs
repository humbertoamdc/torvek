use axum::routing::post;
use axum::Router;

use crate::app_state::AppState;
use crate::orders::controllers::admin_create_order;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/admin/orders", post(admin_create_order))
}
