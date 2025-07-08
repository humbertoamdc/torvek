use crate::projects::models::inputs::GetProjectByIdInput;
use crate::projects::models::project::Project;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetProject {
    projects_repository: Arc<dyn ProjectsRepository>,
}

impl GetProject {
    pub fn new(projects_repository: Arc<dyn ProjectsRepository>) -> Self {
        Self {
            projects_repository,
        }
    }
}

#[async_trait]
impl UseCase<GetProjectByIdInput, Project> for GetProject {
    async fn execute(&self, input: GetProjectByIdInput) -> Result<Project> {
        self.projects_repository
            .get(input.identity.id, input.project_id)
            .await
    }
}
