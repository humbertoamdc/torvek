use crate::app_state::AppState;
use crate::projects::controllers::http::{create_project, query_projects_for_client};
use axum::routing::{get, post};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project))
        .route(
            "/clients/:client_id/projects",
            get(query_projects_for_client),
        )
}
