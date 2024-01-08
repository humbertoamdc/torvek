use crate::projects::domain::errors::ProjectError;
use crate::projects::repositories::projects::ProjectsRepository;
use crate::projects::usecases::UseCase;
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
impl UseCase<CreateProjectRequest, (), ProjectError> for CreateProjectUseCase {
    async fn execute(&self, request: CreateProjectRequest) -> Result<(), ProjectError> {
        let project = Project::new(request.client_id);
        self.projects_repository.create_project(project).await
    }
}
