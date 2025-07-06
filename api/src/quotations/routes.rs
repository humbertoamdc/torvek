use axum::routing::{delete, get, post};
use axum::Router;

use crate::app_state::AppState;
use crate::quotations::controllers::{
    admin_query_quotations_by_status, create_quotation, delete_quotation, download_pdf_quote,
    get_quotation_by_id, get_quotation_subtotal, query_quotations_for_project,
    send_quotation_for_review,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/quotations", post(create_quotation))
        .route(
            "/projects/:project_id/quotations",
            get(query_quotations_for_project),
        )
        .route(
            "/projects/:project_id/quotations/:quotation_id",
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
        .route(
            "/quotations/send_for_review",
            post(send_quotation_for_review),
        )
        .route(
            "/projects/:project_id/quotations/:quotation_id/download_pdf",
            get(download_pdf_quote),
        )
        .route("/admin/quotations", get(admin_query_quotations_by_status))
}
