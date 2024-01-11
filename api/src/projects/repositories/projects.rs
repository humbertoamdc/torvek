use crate::projects::domain::errors::ProjectsError;
use api_boundary::projects::models::Project;
use axum::async_trait;

#[async_trait]
pub trait ProjectsRepository: Send + Sync + 'static {
    async fn create_project(&self, project: Project) -> Result<(), ProjectsError>;
    async fn query_projects_for_client(
        &self,
        client_id: String,
    ) -> Result<Vec<Project>, ProjectsError>;
}
