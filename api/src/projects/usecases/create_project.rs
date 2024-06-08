use crate::projects::repositories::projects::ProjectsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::projects::errors::ProjectsError;
use api_boundary::projects::models::Project;
use api_boundary::projects::requests::CreateProjectRequest;
use axum::async_trait;
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
impl UseCase<CreateProjectRequest, (), ProjectsError> for CreateProjectUseCase {
    async fn execute(&self, request: CreateProjectRequest) -> Result<(), ProjectsError> {
        let project = Project::new(request.client_id);
        self.projects_repository.create_project(project).await
    }
}
