use crate::app_state::AppState;
use crate::projects::controllers::http::create_project;
use axum::routing::post;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/projects", post(create_project))
}
