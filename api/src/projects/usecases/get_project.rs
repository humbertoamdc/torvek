use crate::projects::models::inputs::GetProjectByIdInput;
use crate::projects::models::project::Project;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetProject<P>
where
    P: ProjectsRepository,
{
    projects_repository: Arc<P>,
}

impl<P> GetProject<P>
where
    P: ProjectsRepository,
{
    pub fn new(projects_repository: Arc<P>) -> Self {
        Self {
            projects_repository,
        }
    }
}

#[async_trait]
impl<P> UseCase<GetProjectByIdInput, Project> for GetProject<P>
where
    P: ProjectsRepository,
{
    async fn execute(&self, input: GetProjectByIdInput) -> Result<Project> {
        self.projects_repository
            .get(input.identity.id, input.project_id)
            .await
    }
}
