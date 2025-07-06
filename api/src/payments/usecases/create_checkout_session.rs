use crate::parts::models::inputs::QueryPartsForQuotationInput;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::payments::models::inputs::CreateCheckoutSessionInput;
use crate::services::stripe_client::StripeClient;
use crate::shared::{Result, UseCase};
use api_boundary::common::error::Error;
use api_boundary::payments::responses::CreateCheckoutSessionResponse;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateCheckoutSessionUseCase {
    stripe_client: Arc<dyn StripeClient>,
    query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
}

impl CreateCheckoutSessionUseCase {
    pub const fn new(
        stripe_client: Arc<dyn StripeClient>,
        query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
    ) -> Self {
        Self {
            stripe_client,
            query_parts_for_quotation_usecase,
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
        let query_parts_for_quotation_input = QueryPartsForQuotationInput {
            identity: input.identity.clone(),
            project_id: input.project_id.clone(),
            quotation_id: input.quotation_id.clone(),
            with_quotation_subtotal: false,
            cursor: None,
            limit: 100,
        };

        let parts_for_quotation = self
            .query_parts_for_quotation_usecase
            .execute(query_parts_for_quotation_input)
            .await
            .map_err(|_| Error::UnknownError)?
            .parts;

        let url = self
            .stripe_client
            .create_checkout_session(
                input.identity.id,
                input.project_id,
                input.quotation_id,
                parts_for_quotation,
            )
            .await?;

        Ok(CreateCheckoutSessionResponse::new(url))
    }
}
