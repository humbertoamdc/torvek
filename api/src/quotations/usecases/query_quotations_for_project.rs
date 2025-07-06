use crate::quotations::models::inputs::QueryQuotationsForProjectInput;
use crate::quotations::models::responses::QueryQuotationsForProjectResponse;
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
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
impl UseCase<QueryQuotationsForProjectInput, QueryQuotationsForProjectResponse>
    for QueryQuotationsForProjectUseCase
{
    async fn execute(
        &self,
        input: QueryQuotationsForProjectInput,
    ) -> Result<QueryQuotationsForProjectResponse> {
        let response = self
            .quotations_repository
            .query_quotations_for_project(input.project_id, 100, None)
            .await?;

        Ok(QueryQuotationsForProjectResponse::new(response.data))
    }
}
