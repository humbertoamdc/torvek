use crate::quotations::models::inputs::UpdateQuotationInput;
use crate::quotations::models::quotation::{Quotation, QuoteStatus};
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdateQuotation {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl UpdateQuotation {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<UpdateQuotationInput, Quotation> for UpdateQuotation {
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
