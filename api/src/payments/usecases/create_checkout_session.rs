use crate::payments::services::stripe::StripePaymentsProcessor;
use crate::quotations::usecases::UseCase;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use api_boundary::payments::responses::CreateCheckoutSessionResponse;
use axum::async_trait;
use stripe::StripeError;

pub struct CreateCheckoutSessionUseCase {
    payments_processor: StripePaymentsProcessor,
}

impl CreateCheckoutSessionUseCase {
    pub const fn new(payments_processor: StripePaymentsProcessor) -> Self {
        Self { payments_processor }
    }
}

#[async_trait]
impl UseCase<CreateCheckoutSessionRequest, CreateCheckoutSessionResponse, StripeError>
    for CreateCheckoutSessionUseCase
{
    async fn execute(
        &self,
        request: CreateCheckoutSessionRequest,
    ) -> Result<CreateCheckoutSessionResponse, StripeError> {
        let url = self
            .payments_processor
            .create_checkout_session(request)
            .await?;

        Ok(CreateCheckoutSessionResponse::new(url))
    }
}
