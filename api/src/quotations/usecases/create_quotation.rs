use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use api_boundary::quotations::models::Quotation;
use api_boundary::quotations::requests::CreateQuotationRequest;
use axum::async_trait;
use std::sync::Arc;

pub struct CreateQuotationUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl CreateQuotationUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<CreateQuotationRequest, ()> for CreateQuotationUseCase {
    async fn execute(&self, request: CreateQuotationRequest) -> Result<()> {
        let quotation = Quotation::new(
            request.customer_id,
            request.project_id,
            request.quotation_name,
        );
        self.quotations_repository.create_quotation(quotation).await
    }
}
