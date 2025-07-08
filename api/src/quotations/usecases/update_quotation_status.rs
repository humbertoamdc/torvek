use crate::quotations::models::inputs::SendQuotationForReviewInput;
use crate::quotations::models::quotation::{Quotation, QuotationStatus};
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct SendQuotationForReviewUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl SendQuotationForReviewUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<SendQuotationForReviewInput, Quotation> for SendQuotationForReviewUseCase {
    async fn execute(&self, input: SendQuotationForReviewInput) -> Result<Quotation> {
        self.quotations_repository
            .update(
                input.project_id,
                input.quotation_id,
                Some(QuotationStatus::PendingReview),
            )
            .await
    }
}
