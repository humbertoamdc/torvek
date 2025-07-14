use crate::quotations::models::inputs::GetQuotationByIdInput;
use crate::quotations::models::quotation::Quotation;
use crate::repositories::quotes::QuotesRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetQuotation<Q>
where
    Q: QuotesRepository,
{
    quotations_repository: Arc<Q>,
}

impl<Q> GetQuotation<Q>
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
impl<Q> UseCase<GetQuotationByIdInput, Quotation> for GetQuotation<Q>
where
    Q: QuotesRepository,
{
    async fn execute(&self, input: GetQuotationByIdInput) -> Result<Quotation> {
        self.quotations_repository
            .get(input.identity.id, input.quotation_id)
            .await
    }
}
