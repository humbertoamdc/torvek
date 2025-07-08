use crate::quotations::models::inputs::AdminQueryQuotationsByStatusInput;
use crate::quotations::models::responses::AdminQueryQuotationsByStatusResponse;
use crate::repositories::quotations::{QueryOrderBy, QuotationsRepository};
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct AdminQueryQuotationsByStatusUseCase {
    pub quotations_repository: Arc<dyn QuotationsRepository>,
}

impl AdminQueryQuotationsByStatusUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<AdminQueryQuotationsByStatusInput, AdminQueryQuotationsByStatusResponse>
    for AdminQueryQuotationsByStatusUseCase
{
    async fn execute(
        &self,
        input: AdminQueryQuotationsByStatusInput,
    ) -> Result<AdminQueryQuotationsByStatusResponse> {
        let response = self
            .quotations_repository
            .query(None, Some(input.status), QueryOrderBy::Status, 100, None)
            .await?;

        Ok(AdminQueryQuotationsByStatusResponse::new(response.data))
    }
}
