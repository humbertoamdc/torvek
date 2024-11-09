use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::{Result, UseCase};
use api_boundary::common::money::Money;
use api_boundary::parts::models::{Part, PartQuote};
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::GetQuotationSubtotalRequest;
use api_boundary::quotations::responses::GetQuotationSubtotalResponse;
use axum::async_trait;
use std::sync::Arc;

pub struct GetQuotationSubtotalUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl GetQuotationSubtotalUseCase {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        quotations_repository: Arc<dyn QuotationsRepository>,
    ) -> Self {
        Self {
            parts_repository,
            quotations_repository,
        }
    }
}

#[async_trait]
impl UseCase<GetQuotationSubtotalRequest, GetQuotationSubtotalResponse>
    for GetQuotationSubtotalUseCase
{
    async fn execute(
        &self,
        request: GetQuotationSubtotalRequest,
    ) -> Result<GetQuotationSubtotalResponse> {
        let quotation = self
            .quotations_repository
            .get_quotation_by_id(request.project_id, request.quotation_id.clone())
            .await?;

        if quotation.status != QuotationStatus::PendingPayment {
            return Ok(GetQuotationSubtotalResponse {
                quotation_subtotal: None,
            });
        }

        let response = self
            .parts_repository
            .query_parts_for_quotation(request.quotation_id, 100, None)
            .await?;

        let quotation_subtotal = Some(self.calculate_quotation_subtotal(response.data));

        Ok(GetQuotationSubtotalResponse { quotation_subtotal })
    }
}

impl GetQuotationSubtotalUseCase {
    pub fn calculate_quotation_subtotal(&self, parts: Vec<Part>) -> Money {
        let selected_part_quotes = parts
            .into_iter()
            .map(|part| {
                part.part_quotes
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .find(|part_quote| {
                        part_quote.id == part.selected_part_quote_id.clone().unwrap()
                    })
                    .expect("expecting to have selected part quotes")
            })
            .collect::<Vec<PartQuote>>();

        selected_part_quotes
            .into_iter()
            .fold(Money::default(), |mut money, part_quote| {
                money.amount += part_quote.sub_total.amount;
                money
            })
    }
}
