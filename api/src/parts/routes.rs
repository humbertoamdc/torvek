use axum::routing::{get, patch, post};
use axum::Router;

use crate::app_state::AppState;
use crate::parts::controllers::{
    admin_create_part_quotes, create_drawing_upload_url, create_model_file_upload_url,
    create_parts, get_part, query_parts_for_quotation, update_part, update_selected_part_quote,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/admin/part_quotes", post(admin_create_part_quotes))
        .route("/parts", post(create_parts))
        .route(
            "/customers/:customer_id/projects/:project_id/quotations/:quotation_id/parts",
            get(query_parts_for_quotation),
        )
        .route(
            "/customers/:customer_id/projects/:project_id/quotations/:quotation_id/parts/:part_id",
            get(get_part),
        )
        .route("/parts", patch(update_part))
        .route(
            "/parts/select_part_quote",
            patch(update_selected_part_quote),
        )
        .route(
            "/parts/model_upload_url",
            post(create_model_file_upload_url),
        )
        .route("/parts/drawing_upload_url", post(create_drawing_upload_url))
}
