use crate::parts::models::part::{Part, PartQuote};
use crate::quotations::models::inputs::GetQuotationSubtotalInput;
use crate::quotations::models::quotation::QuoteStatus;
use crate::quotations::models::responses::GetQuotationSubtotalResponse;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::shared::money::Money;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetQuotationSubtotal {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
}

impl GetQuotationSubtotal {
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
impl UseCase<GetQuotationSubtotalInput, GetQuotationSubtotalResponse> for GetQuotationSubtotal {
    async fn execute(
        &self,
        input: GetQuotationSubtotalInput,
    ) -> Result<GetQuotationSubtotalResponse> {
        let quotation = self
            .quotations_repository
            .get(input.project_id, input.quotation_id.clone())
            .await?;

        if quotation.status != QuoteStatus::PendingPayment {
            return Ok(GetQuotationSubtotalResponse {
                quotation_subtotal: None,
            });
        }

        let response = self
            .parts_repository
            .query(input.quotation_id, None, 100)
            .await?;

        let quotation_subtotal = Some(self.calculate_quotation_subtotal(response.data));

        Ok(GetQuotationSubtotalResponse { quotation_subtotal })
    }
}

impl GetQuotationSubtotal {
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
