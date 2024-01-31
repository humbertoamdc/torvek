use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::ConfirmQuotationPaymentWebhookRequest;
use axum::async_trait;
use std::sync::Arc;

pub struct ConfirmQuotationPaymentWebhookUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl ConfirmQuotationPaymentWebhookUseCase {
    pub fn new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<ConfirmQuotationPaymentWebhookRequest, (), QuotationsError>
    for ConfirmQuotationPaymentWebhookUseCase
{
    async fn execute(
        &self,
        request: ConfirmQuotationPaymentWebhookRequest,
    ) -> Result<(), QuotationsError> {
        self.quotations_repository
            .update_quotation_status(
                request.client_id,
                request.project_id,
                request.quotation_id,
                QuotationStatus::Payed,
            )
            .await
    }
}
