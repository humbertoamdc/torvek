use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;
use crate::quotations::controllers::http::{
    admin_query_quotations_by_status, create_quotation, query_quotations_for_project,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/quotations", post(create_quotation))
        .route(
            "/clients/:client_id/projects/:project_id/quotations",
            get(query_quotations_for_project),
        )
        .route("/admin/quotations", get(admin_query_quotations_by_status))
}
