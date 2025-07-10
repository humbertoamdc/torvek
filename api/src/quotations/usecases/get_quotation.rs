use crate::quotations::models::inputs::GetQuotationByIdInput;
use crate::quotations::models::quotation::Quotation;
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetQuotation {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl GetQuotation {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<GetQuotationByIdInput, Quotation> for GetQuotation {
    async fn execute(&self, input: GetQuotationByIdInput) -> Result<Quotation> {
        self.quotations_repository
            .get(input.identity.id, input.quotation_id)
            .await
    }
}
