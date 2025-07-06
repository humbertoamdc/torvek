use crate::payments::models::inputs::CreateCheckoutSessionInput;
use crate::repositories::parts::PartsRepository;
use crate::services::stripe_client::StripeClient;
use crate::shared::{Result, UseCase};
use api_boundary::payments::responses::CreateCheckoutSessionResponse;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateCheckoutSessionUseCase {
    stripe_client: Arc<dyn StripeClient>,
    parts_repository: Arc<dyn PartsRepository>,
}

impl CreateCheckoutSessionUseCase {
    pub const fn new(
        stripe_client: Arc<dyn StripeClient>,
        parts_repository: Arc<dyn PartsRepository>,
    ) -> Self {
        Self {
            stripe_client,
            parts_repository,
        }
    }
}

#[async_trait]
impl UseCase<CreateCheckoutSessionInput, CreateCheckoutSessionResponse>
    for CreateCheckoutSessionUseCase
{
    async fn execute(
        &self,
        input: CreateCheckoutSessionInput,
    ) -> Result<CreateCheckoutSessionResponse> {
        let query_response = self
            .parts_repository
            .query_parts_for_quotation(input.quotation_id.clone(), None, 100)
            .await?;

        let url = self
            .stripe_client
            .create_checkout_session(
                input.identity.id,
                input.project_id,
                input.quotation_id,
                query_response.data,
            )
            .await?;

        Ok(CreateCheckoutSessionResponse::new(url))
    }
}
