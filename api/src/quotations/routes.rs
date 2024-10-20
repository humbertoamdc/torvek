use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;
use crate::quotations::controllers::{
    admin_query_quotations_by_status, create_quotation, get_quotation_by_id,
    query_quotations_for_project,
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
        .route("/admin/quotations", get(admin_query_quotations_by_status))
}
