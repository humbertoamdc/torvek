use std::sync::Arc;

use axum::async_trait;

use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::StripeConfirmQuotationPaymentRequest;

use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::shared::usecase::UseCase;

pub struct ConfirmQuotationPaymentWebhookUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl ConfirmQuotationPaymentWebhookUseCase {
    pub fn _new(quotations_repository: Arc<dyn QuotationsRepository>) -> Self {
        Self {
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<StripeConfirmQuotationPaymentRequest, (), QuotationsError>
    for ConfirmQuotationPaymentWebhookUseCase
{
    async fn execute(
        &self,
        request: StripeConfirmQuotationPaymentRequest,
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
