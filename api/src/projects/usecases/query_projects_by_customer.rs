use crate::projects::models::inputs::QueryProjectsForClientInput;
use crate::projects::models::responses::QueryProjectsForClientResponse;
use crate::repositories::projects::ProjectsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct QueryProjectsByCustomer {
    projects_repository: Arc<dyn ProjectsRepository>,
}

impl QueryProjectsByCustomer {
    pub fn new(projects_repository: Arc<dyn ProjectsRepository>) -> Self {
        Self {
            projects_repository,
        }
    }
}

#[async_trait]
impl UseCase<QueryProjectsForClientInput, QueryProjectsForClientResponse>
    for QueryProjectsByCustomer
{
    async fn execute(
        &self,
        input: QueryProjectsForClientInput,
    ) -> Result<QueryProjectsForClientResponse> {
        let response = self
            .projects_repository
            .query(input.identity.id, None, 100)
            .await?;

        Ok(QueryProjectsForClientResponse {
            projects: response.data,
            cursor: response.cursor,
        })
    }
}
