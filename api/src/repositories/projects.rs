use crate::shared::{QueryResponse, Result};
use api_boundary::projects::models::Project;
use axum::async_trait;

#[async_trait]
pub trait ProjectsRepository: Send + Sync + 'static {
    async fn create_project(&self, project: Project) -> Result<()>;
    async fn query_projects_for_client(
        &self,
        customer_id: String,
        page_limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Project>, String>>;
    async fn get_project_by_id(&self, customer_id: String, project_id: String) -> Result<Project>;

    /// Delete project ONLY if it is not in `LOCKED` status.
    async fn try_delete_project(&self, customer_id: String, project_id: String) -> Result<()>;
}
