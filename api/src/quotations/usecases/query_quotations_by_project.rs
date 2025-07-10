use crate::quotations::models::inputs::QueryQuotationsForProjectInput;
use crate::quotations::models::responses::QueryQuotationsForProjectResponse;
use crate::repositories::quotations::{QueryBy, QuotationsRepository};
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct QueryQuotationsByProject {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl QueryQuotationsByProject {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<QueryQuotationsForProjectInput, QueryQuotationsForProjectResponse>
    for QueryQuotationsByProject
{
    async fn execute(
        &self,
        input: QueryQuotationsForProjectInput,
    ) -> Result<QueryQuotationsForProjectResponse> {
        let response = self
            .quotations_repository
            .query(
                Some(input.identity.id),
                Some(input.project_id),
                None,
                None,
                None,
                QueryBy::Customer,
                100,
                None,
            )
            .await?;

        Ok(QueryQuotationsForProjectResponse::new(response.data))
    }
}
