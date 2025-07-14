use crate::payments::models::inputs::CreateCheckoutSessionInput;
use crate::payments::models::responses::CreateCheckoutSessionResponse;
use crate::repositories::parts::PartsRepository;
use crate::services::stripe_client::StripeClient;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateCheckoutSession<P>
where
    P: PartsRepository,
{
    stripe_client: Arc<dyn StripeClient>,
    parts_repository: Arc<P>,
}

impl<P> CreateCheckoutSession<P>
where
    P: PartsRepository,
{
    pub const fn new(stripe_client: Arc<dyn StripeClient>, parts_repository: Arc<P>) -> Self {
        Self {
            stripe_client,
            parts_repository,
        }
    }
}

#[async_trait]
impl<P> UseCase<CreateCheckoutSessionInput, CreateCheckoutSessionResponse>
    for CreateCheckoutSession<P>
where
    P: PartsRepository,
{
    async fn execute(
        &self,
        input: CreateCheckoutSessionInput,
    ) -> Result<CreateCheckoutSessionResponse> {
        let query_response = self
            .parts_repository
            .query(
                input.identity.id.clone(),
                input.quotation_id.clone(),
                None,
                100,
            )
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
