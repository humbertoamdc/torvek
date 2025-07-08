use crate::projects::models::inputs::GetProjectByIdInput;
use crate::projects::models::project::Project;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetProjectByIdUseCase {
    projects_repository: Arc<dyn ProjectsRepository>,
}

impl GetProjectByIdUseCase {
    pub fn new(projects_repository: Arc<dyn ProjectsRepository>) -> Self {
        Self {
            projects_repository,
        }
    }
}

#[async_trait]
impl UseCase<GetProjectByIdInput, Project> for GetProjectByIdUseCase {
    async fn execute(&self, input: GetProjectByIdInput) -> Result<Project> {
        self.projects_repository
            .get(input.identity.id, input.project_id)
            .await
    }
}
