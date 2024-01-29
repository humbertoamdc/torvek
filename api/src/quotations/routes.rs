use crate::app_state::AppState;
use crate::quotations::controllers::http::{
    confirm_quotation_payment_webhook, create_quotation, query_quotations_for_project,
};
use axum::routing::{get, post};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/quotations", post(create_quotation))
        .route(
            "/clients/:client_id/projects/:project_id/quotations",
            get(query_quotations_for_project),
        )
        .route(
            "/quotations/webhooks/confirm_payment",
            post(confirm_quotation_payment_webhook),
        )
}
