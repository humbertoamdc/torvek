use crate::projects::models::requests::GetProjectByIdInput;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use api_boundary::projects::models::Project;
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
            .get_project_by_id(input.identity.id, input.project_id)
            .await
    }
}
