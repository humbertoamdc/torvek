use crate::app_state::AppState;
use crate::projects::controllers::{
    create_project, delete_project, get_project_by_id, query_projects_by_customer,
};
use axum::routing::{delete, get, post};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project))
        .route("/projects", get(query_projects_by_customer))
        .route("/projects/:project_id", get(get_project_by_id))
        .route("/projects/:project_id", delete(delete_project))
}
