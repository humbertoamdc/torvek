use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::app_state::AppState;
use crate::parts::controllers::{
    admin_create_part_quotes, admin_query_parts_for_quotation, create_parts, delete_drawing_file,
    delete_part, generate_presigned_url, get_part, query_parts_for_quotation, update_part,
    update_selected_part_quote, upload_part_drawing,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/admin/part_quotes", post(admin_create_part_quotes))
        .route(
            "/admin/customers/:customer_id/quotations/:quotation_id/parts",
            get(admin_query_parts_for_quotation),
        )
        .route("/parts", post(create_parts))
        .route(
            "/projects/:project_id/quotations/:quotation_id/parts",
            get(query_parts_for_quotation),
        )
        .route("/parts/:part_id", get(get_part))
        .route("/parts", patch(update_part))
        .route(
            "/parts/select_part_quote",
            patch(update_selected_part_quote),
        )
        .route(
            "/projects/:project_id/quotations/:quotation_id/parts/:part_id",
            delete(delete_part),
        )
        .route(
            "/projects/:project_id/quotations/:quotation_id/parts/:part_id/files/drawing",
            post(upload_part_drawing),
        )
        .route(
            "/projects/:project_id/quotations/:quotation_id/parts/:part_id/files/drawing",
            delete(delete_drawing_file),
        )
        .route("/presigned_url", post(generate_presigned_url))
}
