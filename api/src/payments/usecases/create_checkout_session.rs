use api_boundary::common::error::Error;
use axum::async_trait;
use std::sync::Arc;

use api_boundary::parts::requests::QueryPartsForQuotationRequest;
use api_boundary::payments::responses::CreateCheckoutSessionResponse;

use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::payments::domain::requests::CreateCheckoutSessionRequest;
use crate::services::payment_processor::PaymentsProcessor;
use crate::shared::{Result, UseCase};

pub struct CreateCheckoutSessionUseCase {
    payments_processor: Arc<dyn PaymentsProcessor>,
    query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
}

impl CreateCheckoutSessionUseCase {
    pub const fn new(
        payments_processor: Arc<dyn PaymentsProcessor>,
        query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
    ) -> Self {
        Self {
            payments_processor,
            query_parts_for_quotation_usecase,
        }
    }
}

#[async_trait]
impl UseCase<CreateCheckoutSessionRequest, CreateCheckoutSessionResponse>
    for CreateCheckoutSessionUseCase
{
    async fn execute(
        &self,
        request: CreateCheckoutSessionRequest,
    ) -> Result<CreateCheckoutSessionResponse> {
        let query_parts_for_quotation_request = QueryPartsForQuotationRequest {
            quotation_id: request.quotation_id.clone(),
            with_quotation_subtotal: false,
        };

        let parts_for_quotation = self
            .query_parts_for_quotation_usecase
            .execute(query_parts_for_quotation_request)
            .await
            .map_err(|_| Error::UnknownError)?
            .parts;

        let url = self
            .payments_processor
            .create_checkout_session(
                request.customer_id,
                request.project_id,
                request.quotation_id,
                parts_for_quotation,
            )
            .await?;

        Ok(CreateCheckoutSessionResponse::new(url))
    }
}
