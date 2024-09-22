use crate::app_state::AppState;
use crate::projects::controllers::http::{
    create_project, get_project_by_id, query_projects_for_client,
};
use axum::routing::{get, post};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project))
        .route(
            "/customers/:customer_id/projects",
            get(query_projects_for_client),
        )
        .route(
            "/customers/:customer_id/projects/:project_id",
            get(get_project_by_id),
        )
}
