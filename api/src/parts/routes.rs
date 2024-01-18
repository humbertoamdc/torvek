use crate::app_state::AppState;
use crate::parts::controllers::http::{
    create_drawing_upload_url, create_parts, query_parts_for_quotation, update_part,
};
use axum::routing::{get, patch, post};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/parts", post(create_parts))
        .route(
            "/clients/:client_id/projects/:project_id/quotations/:quotation_id/parts",
            get(query_parts_for_quotation),
        )
        .route("/parts", patch(update_part))
        .route("/parts/drawing_upload_url", post(create_drawing_upload_url))
}
