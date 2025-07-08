use crate::projects::models::inputs::CreateProjectInput;
use crate::projects::models::project::Project;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateProjectUseCase {
    projects_repository: Arc<dyn ProjectsRepository>,
}

impl CreateProjectUseCase {
    pub fn new(projects_repository: Arc<dyn ProjectsRepository>) -> Self {
        Self {
            projects_repository,
        }
    }
}

#[async_trait]
impl UseCase<CreateProjectInput, ()> for CreateProjectUseCase {
    async fn execute(&self, input: CreateProjectInput) -> Result<()> {
        let project = Project::new(input.identity.id, input.project_name);
        self.projects_repository.create(project).await
    }
}
