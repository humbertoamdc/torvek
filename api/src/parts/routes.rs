use crate::app_state::AppState;
use crate::parts::controllers::http::create_parts;
use axum::routing::post;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/parts", post(create_parts))
}
