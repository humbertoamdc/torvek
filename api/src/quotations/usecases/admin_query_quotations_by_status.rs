use crate::quotations::models::inputs::AdminQueryQuotationsByStatusInput;
use crate::quotations::models::responses::AdminQueryQuotationsByStatusResponse;
use crate::repositories::quotations::{QueryBy, QuotationsRepository};
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct AdminQueryQuotationsByStatus {
    pub quotations_repository: Arc<dyn QuotationsRepository>,
}

impl AdminQueryQuotationsByStatus {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<AdminQueryQuotationsByStatusInput, AdminQueryQuotationsByStatusResponse>
    for AdminQueryQuotationsByStatus
{
    async fn execute(
        &self,
        input: AdminQueryQuotationsByStatusInput,
    ) -> Result<AdminQueryQuotationsByStatusResponse> {
        let response = self
            .quotations_repository
            .query(
                None,
                None,
                None,
                None,
                Some(input.status),
                QueryBy::IsPendingReview,
                100,
                None,
            )
            .await?;

        Ok(AdminQueryQuotationsByStatusResponse::new(response.data))
    }
}
