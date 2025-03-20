use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use api_boundary::quotations::requests::AdminQueryQuotationsByStatusRequest;
use api_boundary::quotations::responses::AdminQueryQuotationsByStatusResponse;
use async_trait::async_trait;
use std::sync::Arc;

pub struct AdminQueryQuotationsByStatusUseCase {
    pub quotations_repository: Arc<dyn QuotationsRepository>,
}

impl AdminQueryQuotationsByStatusUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<AdminQueryQuotationsByStatusRequest, AdminQueryQuotationsByStatusResponse>
    for AdminQueryQuotationsByStatusUseCase
{
    async fn execute(
        &self,
        request: AdminQueryQuotationsByStatusRequest,
    ) -> Result<AdminQueryQuotationsByStatusResponse> {
        let response = self
            .quotations_repository
            .query_quotations_by_status(request.status, 100, None)
            .await?;

        Ok(AdminQueryQuotationsByStatusResponse::new(response.data))
    }
}
