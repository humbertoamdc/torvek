use axum::routing::{get, patch, post};
use axum::Router;

use crate::app_state::AppState;
use crate::orders::adapters::api::controllers::{
    admin_query_orders_by_status, admin_update_order, create_drawing_upload_url,
    create_orders_and_file_upload_urls, query_orders_for_client, update_order,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/clients/:client_id/orders", get(query_orders_for_client))
        .route("/orders", post(create_orders_and_file_upload_urls))
        .route("/orders", patch(update_order))
        .route(
            "/orders/drawing_upload_url",
            post(create_drawing_upload_url),
        )
        .route("/admin/orders", get(admin_query_orders_by_status))
        .route("/admin/orders", patch(admin_update_order))
}
