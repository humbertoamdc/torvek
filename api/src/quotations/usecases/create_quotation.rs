use crate::quotations::models::inputs::CreateQuotationInput;
use crate::quotations::models::quotation::Quotation;
use crate::repositories::quotes::QuotesRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateQuotation {
    quotations_repository: Arc<dyn QuotesRepository>,
}

impl CreateQuotation {
    pub fn new(quotations_repository: Arc<dyn QuotesRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<CreateQuotationInput, ()> for CreateQuotation {
    async fn execute(&self, input: CreateQuotationInput) -> Result<()> {
        let quotation = Quotation::new(input.identity.id, input.project_id, input.quotation_name);
        self.quotations_repository.create(quotation).await
    }
}
