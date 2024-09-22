use crate::projects::repositories::projects::ProjectsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::projects::errors::ProjectsError;
use api_boundary::projects::models::Project;
use api_boundary::projects::requests::GetProjectByIdRequest;
use axum::async_trait;
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
impl UseCase<GetProjectByIdRequest, Project, ProjectsError> for GetProjectByIdUseCase {
    async fn execute(&self, request: GetProjectByIdRequest) -> Result<Project, ProjectsError> {
        self.projects_repository
            .get_project_by_id(request.customer_id, request.project_id)
            .await
    }
}
