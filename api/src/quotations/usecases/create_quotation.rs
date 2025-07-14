use crate::quotations::models::inputs::CreateQuotationInput;
use crate::quotations::models::quotation::Quotation;
use crate::repositories::quotes::QuotesRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateQuotation<Q>
where
    Q: QuotesRepository,
{
    quotations_repository: Arc<Q>,
}

impl<Q> CreateQuotation<Q>
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
impl<Q> UseCase<CreateQuotationInput, ()> for CreateQuotation<Q>
where
    Q: QuotesRepository,
{
    async fn execute(&self, input: CreateQuotationInput) -> Result<()> {
        let quotation = Quotation::new(input.identity.id, input.project_id, input.quotation_name);
        self.quotations_repository.create(quotation).await
    }
}
