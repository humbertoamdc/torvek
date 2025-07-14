use crate::projects::models::inputs::CreateProjectInput;
use crate::projects::models::project::Project;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateProject<P>
where
    P: ProjectsRepository,
{
    projects_repository: Arc<P>,
}

impl<P> CreateProject<P>
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
impl<P> UseCase<CreateProjectInput, ()> for CreateProject<P>
where
    P: ProjectsRepository,
{
    async fn execute(&self, input: CreateProjectInput) -> Result<()> {
        let project = Project::new(input.identity.id, input.project_name);
        self.projects_repository.create(project).await
    }
}
