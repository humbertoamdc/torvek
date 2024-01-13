use crate::app_state::AppState;
use crate::quotations::controllers::http::create_quotation;
use axum::routing::post;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/quotations", post(create_quotation))
}
