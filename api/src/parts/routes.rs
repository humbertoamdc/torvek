use axum::routing::{get, patch, post};
use axum::Router;

use crate::app_state::AppState;
use crate::parts::controllers::http::{
    admin_create_part_quotes, admin_update_part, create_drawing_upload_url, create_parts,
    query_parts_for_quotation, update_part,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/parts", post(create_parts))
        .route(
            "/clients/:client_id/projects/:project_id/quotations/:quotation_id/parts",
            get(query_parts_for_quotation),
        )
        .route("/parts", patch(update_part))
        .route("/parts/drawing_upload_url", post(create_drawing_upload_url))
        .route("/admin/parts", patch(admin_update_part))
        .route("/admin/part_quotes", post(admin_create_part_quotes))
}
