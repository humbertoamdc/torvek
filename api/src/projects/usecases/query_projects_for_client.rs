use crate::projects::models::requests::QueryProjectsForClientInput;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use api_boundary::projects::responses::QueryProjectsForClientResponse;
use async_trait::async_trait;
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
impl UseCase<QueryProjectsForClientInput, QueryProjectsForClientResponse>
    for QueryProjectsForClientUseCase
{
    async fn execute(
        &self,
        input: QueryProjectsForClientInput,
    ) -> Result<QueryProjectsForClientResponse> {
        let response = self
            .projects_repository
            .query_projects_for_client(input.identity.id, 100, None)
            .await?;

        Ok(QueryProjectsForClientResponse {
            projects: response.data,
            cursor: response.cursor,
        })
    }
}
