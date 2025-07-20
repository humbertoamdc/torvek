use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::app_state::AppState;
use crate::parts::controllers::{
    admin_create_part_quotes, admin_query_parts_for_quotation, create_drawing_upload_url,
    create_model_file_upload_url, create_parts, delete_part, generate_presigned_url, get_part,
    query_parts_for_quotation, update_part, update_selected_part_quote,
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
        .route(
            "/projects/:project_id/quotations/:quotation_id/parts/:part_id",
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
        .route(
            "/projects/:project_id/quotations/:quotation_id/parts/:part_id",
            delete(delete_part),
        )
        .route("/presigned_url", post(generate_presigned_url))
}
