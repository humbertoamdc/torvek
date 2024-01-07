use crate::projects::domain::errors::ProjectError;
use api_boundary::projects::models::Project;
use axum::async_trait;

#[async_trait]
pub trait ProjectsRepository: Send + Sync + 'static {
    async fn create_project(&self, project: Project) -> Result<(), ProjectError>;
}
