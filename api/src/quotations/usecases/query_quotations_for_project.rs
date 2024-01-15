use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::quotations::usecases::UseCase;
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
impl UseCase<QueryQuotationsForProjectRequest, QueryQuotationsForProjectResponse, QuotationsError>
    for QueryQuotationsForProjectUseCase
{
    async fn execute(
        &self,
        request: QueryQuotationsForProjectRequest,
    ) -> Result<QueryQuotationsForProjectResponse, QuotationsError> {
        let quotations = self
            .quotations_repository
            .query_quotations_for_project(request.client_id, request.project_id)
            .await?;

        Ok(QueryQuotationsForProjectResponse::new(quotations))
    }
}
