use api_boundary::parts::models::PartQuote;
use axum::async_trait;
use std::collections::HashMap;

use crate::parts::usecases::query_part_quotes_for_parts::QueryPartQuotesForPartsUseCase;
use api_boundary::parts::requests::{
    QueryPartQuotesForPartsRequest, QueryPartsForQuotationRequest,
};
use api_boundary::payments::errors::PaymentsError;
use api_boundary::payments::requests::CreateCheckoutSessionRequest;
use api_boundary::payments::responses::CreateCheckoutSessionResponse;

use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::payments::services::stripe::StripePaymentsProcessor;
use crate::shared::usecase::UseCase;

pub struct CreateCheckoutSessionUseCase {
    payments_processor: StripePaymentsProcessor,
    query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
    query_part_quotes_for_parts_use_case: QueryPartQuotesForPartsUseCase,
}

impl CreateCheckoutSessionUseCase {
    pub const fn new(
        payments_processor: StripePaymentsProcessor,
        query_parts_for_quotation_usecase: QueryPartsForQuotationUseCase,
        query_part_quotes_for_parts_use_case: QueryPartQuotesForPartsUseCase,
    ) -> Self {
        Self {
            payments_processor,
            query_parts_for_quotation_usecase,
            query_part_quotes_for_parts_use_case,
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
            .map_err(|_| PaymentsError::UnknownError)?;

        let part_ids = request
            .selected_quotes_per_part
            .iter()
            .map(|(part_id, _)| part_id.clone())
            .collect::<Vec<String>>();
        let query_part_quotes_for_parts_request = QueryPartQuotesForPartsRequest { part_ids };
        // TODO: Implement a batch get endpoint to retrieve only the selected part quote per part.
        let query_part_quotes_for_parts_response = self
            .query_part_quotes_for_parts_use_case
            .execute(query_part_quotes_for_parts_request)
            .await
            .map_err(|_| PaymentsError::UnknownError)?;
        let selected_quote_per_part = query_part_quotes_for_parts_response
            .part_quotes_by_part_id
            .iter()
            .map(|(part_id, part_quotes)| {
                let target_part_quote_id =
                    request.selected_quotes_per_part[&part_id.clone()].clone();
                let target_part_quote = part_quotes
                    .iter()
                    .find(|part_quote| part_quote.id == target_part_quote_id)
                    .unwrap()
                    .clone();
                (part_id.clone(), target_part_quote)
            })
            .collect::<HashMap<String, PartQuote>>();

        let url = self
            .payments_processor
            .create_checkout_session(
                request.client_id,
                request.project_id,
                request.quotation_id,
                query_parts_for_quotation_response.parts,
                selected_quote_per_part,
            )
            .await?;

        Ok(CreateCheckoutSessionResponse::new(url))
    }
}
