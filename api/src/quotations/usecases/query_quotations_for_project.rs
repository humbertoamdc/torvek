use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use api_boundary::quotations::requests::QueryQuotationsForProjectRequest;
use api_boundary::quotations::responses::QueryQuotationsForProjectResponse;
use axum::async_trait;
use std::sync::Arc;

pub struct QueryQuotationsForProjectUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl QueryQuotationsForProjectUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<QueryQuotationsForProjectRequest, QueryQuotationsForProjectResponse>
    for QueryQuotationsForProjectUseCase
{
    async fn execute(
        &self,
        request: QueryQuotationsForProjectRequest,
    ) -> Result<QueryQuotationsForProjectResponse> {
        let response = self
            .quotations_repository
            .query_quotations_for_project(request.project_id, 100, None)
            .await?;

        Ok(QueryQuotationsForProjectResponse::new(response.data))
    }
}
