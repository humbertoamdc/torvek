use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::quotations::models::Quotation;
use api_boundary::quotations::requests::GetQuotationByIdRequest;
use axum::async_trait;
use std::sync::Arc;

pub struct GetQuotationByIdUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl GetQuotationByIdUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<GetQuotationByIdRequest, Quotation, QuotationsError> for GetQuotationByIdUseCase {
    async fn execute(
        &self,
        request: GetQuotationByIdRequest,
    ) -> Result<Quotation, QuotationsError> {
        self.quotations_repository
            .get_quotation_by_id(request.client_id, request.project_id, request.quotation_id)
            .await
    }
}
