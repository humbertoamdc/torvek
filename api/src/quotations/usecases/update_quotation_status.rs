use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::quotations::requests::UpdateQuotationStatusRequest;
use axum::async_trait;
use std::sync::Arc;

pub struct UpdateQuotationStatusUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl UpdateQuotationStatusUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<UpdateQuotationStatusRequest, (), QuotationsError> for UpdateQuotationStatusUseCase {
    async fn execute(&self, request: UpdateQuotationStatusRequest) -> Result<(), QuotationsError> {
        self.quotations_repository
            .update_quotation_status(
                request.client_id,
                request.project_id,
                request.quotation_id,
                request.status,
            )
            .await
    }
}
