use crate::projects::repositories::projects::ProjectsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::projects::errors::ProjectsError;
use api_boundary::projects::requests::QueryProjectsForClientRequest;
use api_boundary::projects::responses::QueryProjectsForClientResponse;
use axum::async_trait;
use std::sync::Arc;

pub struct QueryProjectsForClientUseCase {
    projects_repository: Arc<dyn ProjectsRepository>,
}

impl QueryProjectsForClientUseCase {
    pub fn new(projects_repository: Arc<dyn ProjectsRepository>) -> Self {
        Self {
            projects_repository,
        }
    }
}

#[async_trait]
impl UseCase<QueryProjectsForClientRequest, QueryProjectsForClientResponse, ProjectsError>
    for QueryProjectsForClientUseCase
{
    async fn execute(
        &self,
        request: QueryProjectsForClientRequest,
    ) -> Result<QueryProjectsForClientResponse, ProjectsError> {
        let projects = self
            .projects_repository
            .query_projects_for_client(request.customer_id)
            .await?;

        Ok(QueryProjectsForClientResponse::new(projects))
    }
}
