use crate::quotations::models::inputs::UpdateQuotationInput;
use crate::quotations::models::quotation::{Quotation, QuoteStatus};
use crate::repositories::quotes::QuotesRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdateQuotation<Q>
where
    Q: QuotesRepository,
{
    quotations_repository: Arc<Q>,
}

impl<Q> UpdateQuotation<Q>
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
impl<Q> UseCase<UpdateQuotationInput, Quotation> for UpdateQuotation<Q>
where
    Q: QuotesRepository,
{
    async fn execute(&self, input: UpdateQuotationInput) -> Result<Quotation> {
        self.quotations_repository
            .update(
                input.identity.id,
                input.project_id,
                input.quotation_id,
                Some(QuoteStatus::PendingReview),
            )
            .await
    }
}
