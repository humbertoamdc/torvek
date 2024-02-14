use axum::async_trait;

use api_boundary::parts::requests::QueryPartsForQuotationRequest;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use api_boundary::payments::responses::CreateCheckoutSessionResponse;

use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::payments::domain::errors::PaymentsError;
use crate::payments::services::stripe::StripePaymentsProcessor;
use crate::shared::usecase::UseCase;

pub struct CreateCheckoutSessionUseCase {
    payments_processor: StripePaymentsProcessor,
    query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
}

impl CreateCheckoutSessionUseCase {
    pub const fn new(
        payments_processor: StripePaymentsProcessor,
        query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
    ) -> Self {
        Self {
            payments_processor,
            query_parts_for_quotation_usecase,
        }
    }
}

#[async_trait]
impl UseCase<CreateCheckoutSessionRequest, CreateCheckoutSessionResponse, PaymentsError>
    for CreateCheckoutSessionUseCase
{
    async fn execute(
        &self,
        request: CreateCheckoutSessionRequest,
    ) -> Result<CreateCheckoutSessionResponse, PaymentsError> {
        let query_parts_for_quotation_request = QueryPartsForQuotationRequest::new(
            request.client_id.clone(),
            request.project_id.clone(),
            request.quotation_id.clone(),
        );
        let query_parts_for_quotation_response = self
            .query_parts_for_quotation_usecase
            .execute(query_parts_for_quotation_request)
            .await
            .map_err(|_| PaymentsError::QueryPartsError)?;

        let url = self
            .payments_processor
            .create_checkout_session(
                request.client_id,
                request.project_id,
                request.quotation_id,
                query_parts_for_quotation_response.parts,
            )
            .await?;

        Ok(CreateCheckoutSessionResponse::new(url))
    }
}
