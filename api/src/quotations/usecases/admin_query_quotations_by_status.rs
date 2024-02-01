use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::quotations::requests::AdminQueryQuotationsByStatusRequest;
use api_boundary::quotations::responses::AdminQueryQuotationsByStatusResponse;
use axum::async_trait;
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
impl
    UseCase<
        AdminQueryQuotationsByStatusRequest,
        AdminQueryQuotationsByStatusResponse,
        QuotationsError,
    > for AdminQueryQuotationsByStatusUseCase
{
    async fn execute(
        &self,
        request: AdminQueryQuotationsByStatusRequest,
    ) -> Result<AdminQueryQuotationsByStatusResponse, QuotationsError> {
        let quotations = self
            .quotations_repository
            .query_quotations_by_status(request.status)
            .await?;

        Ok(AdminQueryQuotationsByStatusResponse::new(quotations))
    }
}
