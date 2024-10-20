use crate::shared::Result;
use api_boundary::projects::models::Project;
use axum::async_trait;

#[async_trait]
pub trait ProjectsRepository: Send + Sync + 'static {
    async fn create_project(&self, project: Project) -> Result<()>;
    async fn query_projects_for_client(&self, customer_id: String) -> Result<Vec<Project>>;
    async fn get_project_by_id(&self, customer_id: String, project_id: String) -> Result<Project>;
}
