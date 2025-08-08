use crate::app_state::AppState;
use crate::landing::controllers::contact_admins;
use axum::routing::post;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/contact", post(contact_admins))
}
