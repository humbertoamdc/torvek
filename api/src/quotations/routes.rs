use axum::routing::{delete, get, post};
use axum::Router;

use crate::app_state::AppState;
use crate::quotations::controllers::{
    admin_query_quotations_by_status, create_quotation, delete_quotation, get_quotation_by_id,
    get_quotation_subtotal, query_quotations_for_project,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/quotations", post(create_quotation))
        .route(
            "/customers/:customer_id/projects/:project_id/quotations",
            get(query_quotations_for_project),
        )
        .route(
            "/customers/:customer_id/projects/:project_id/quotations/:quotation_id",
            get(get_quotation_by_id),
        )
        .route(
            "/projects/:project_id/quotations/:quotation_id/subtotal",
            get(get_quotation_subtotal),
        )
        .route(
            "/projects/:project_id/quotations/:quotation_id",
            delete(delete_quotation),
        )
        .route("/admin/quotations", get(admin_query_quotations_by_status))
}
