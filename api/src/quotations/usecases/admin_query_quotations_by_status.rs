use crate::quotations::models::inputs::AdminQueryQuotationsByStatusInput;
use crate::quotations::models::responses::AdminQueryQuotationsByStatusResponse;
use crate::repositories::quotes::{QueryBy, QuotesRepository};
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct AdminQueryQuotationsByStatus<Q>
where
    Q: QuotesRepository,
{
    pub quotations_repository: Arc<Q>,
}

impl<Q> AdminQueryQuotationsByStatus<Q>
where
    Q: QuotesRepository,
{
    pub fn new(quotations_repository: Arc<Q>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl<Q> UseCase<AdminQueryQuotationsByStatusInput, AdminQueryQuotationsByStatusResponse>
    for AdminQueryQuotationsByStatus<Q>
where
    Q: QuotesRepository,
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
