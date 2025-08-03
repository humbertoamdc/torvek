use crate::quotations::models::inputs::SendForReviewInput;
use crate::quotations::models::quotation::{Quotation, QuoteStatus};
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::services::emailer::Emailer;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct SendForReview<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    quotations_repository: Arc<Q>,
    parts_repository: Arc<P>,
    emailer_service: Arc<dyn Emailer>,
}

impl<Q, P> SendForReview<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    pub fn new(
        quotations_repository: Arc<Q>,
        parts_repository: Arc<P>,
        emailer_service: Arc<dyn Emailer>,
    ) -> Self {
        Self {
            quotations_repository,
            parts_repository,
            emailer_service,
        }
    }
}

#[async_trait]
impl<Q, P> UseCase<SendForReviewInput, Quotation> for SendForReview<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    async fn execute(&self, input: SendForReviewInput) -> Result<Quotation> {
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

        let quotation = self
            .quotations_repository
            .update_status(
                input.identity.id.clone(),
                input.project_id,
                input.quotation_id.clone(),
                QuoteStatus::PendingReview,
            )
            .await?;

        let _ = self
            .emailer_service
            .send_email_to_admins(
                "A quote needs review",
                &format!(
                    "Customer with id {} sent the quote with id {} for review.",
                    input.identity.id, input.quotation_id
                ),
            )
            .await;

        Ok(quotation)
    }
}
