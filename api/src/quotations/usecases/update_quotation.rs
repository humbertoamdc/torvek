use crate::quotations::models::inputs::UpdateQuotationInput;
use crate::quotations::models::quotation::{Quotation, QuoteStatus};
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdateQuotation<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    quotations_repository: Arc<Q>,
    parts_repository: Arc<P>,
}

impl<Q, P> UpdateQuotation<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    pub fn new(quotations_repository: Arc<Q>, parts_repository: Arc<P>) -> Self {
        Self {
            quotations_repository,
            parts_repository,
        }
    }
}

#[async_trait]
impl<Q, P> UseCase<UpdateQuotationInput, Quotation> for UpdateQuotation<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    async fn execute(&self, input: UpdateQuotationInput) -> Result<Quotation> {
        let query_response = self
            .parts_repository
            .query(
                input.identity.id.clone(),
                input.quotation_id.clone(),
                None,
                100,
            )
            .await?;

        query_response
            .data
            .iter()
            .try_for_each(|part| part.validate())?;

        self.quotations_repository
            .update_status(
                input.identity.id,
                input.project_id,
                input.quotation_id,
                QuoteStatus::PendingReview,
            )
            .await
    }
}
