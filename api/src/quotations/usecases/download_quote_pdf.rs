use crate::quotations::models::inputs::DownloadQuotePdfInput;
use crate::quotations::models::quotation::{Quotation, QuoteStatus};
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::services::stripe_client::{PriceData, QuoteLineItem, StripeClient};
use crate::shared;
use crate::shared::error::Error;
use crate::shared::{CustomerId, QuoteId, UseCase};
use async_trait::async_trait;
use axum::body::Bytes;
use iso_currency::Currency;
use shared::Result;
use std::sync::Arc;

pub struct DowanloadQuotePdf {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
    stripe_client: Arc<dyn StripeClient>,
}

impl DowanloadQuotePdf {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        quotations_repository: Arc<dyn QuotationsRepository>,
        stripe_client: Arc<dyn StripeClient>,
    ) -> Self {
        Self {
            parts_repository,
            quotations_repository,
            stripe_client,
        }
    }
}

#[async_trait]
impl UseCase<DownloadQuotePdfInput, Bytes> for DowanloadQuotePdf {
    async fn execute(&self, input: DownloadQuotePdfInput) -> crate::shared::Result<Bytes> {
        let quote = self
            .quotations_repository
            .get(input.identity.id.clone(), input.quotation_id.clone())
            .await?;

        if !self.is_valid_quote_status(quote).await {
            return Err(Error::NoPdfQuoteAvailable);
        }

        let quote_line_items = self
            .generate_quote_line_items(input.identity.id, input.quotation_id)
            .await?;

        let stripe_customer_id = input
            .identity
            .metadata_public
            .stripe_customer_id
            .expect("No stripe-customer ID configured");

        let stripe_quote = self
            .stripe_client
            .create_quote(stripe_customer_id, quote_line_items)
            .await?;

        self.stripe_client
            .finalize_quote(stripe_quote.id.clone())
            .await?;

        self.stripe_client.download_quote_pdf(stripe_quote.id).await
    }
}

impl DowanloadQuotePdf {
    async fn is_valid_quote_status(&self, quote: Quotation) -> bool {
        quote.status == QuoteStatus::PendingPayment || quote.status == QuoteStatus::Payed
    }

    async fn generate_quote_line_items(
        &self,
        customer_id: CustomerId,
        quotation_id: QuoteId,
    ) -> Result<Vec<QuoteLineItem>> {
        let page_limit = 100;
        let query_part_response = self
            .parts_repository
            .query(customer_id, quotation_id, None, page_limit)
            .await?;

        let quote_line_items = query_part_response
            .data
            .into_iter()
            .map(|part| {
                let selected_part_quote = part
                    .part_quotes
                    .unwrap()
                    .iter()
                    .find(|part_quote| {
                        part.selected_part_quote_id.clone() == Some(part_quote.id.clone())
                    })
                    .unwrap()
                    .clone();

                QuoteLineItem {
                    price_data: PriceData {
                        currency: match selected_part_quote.sub_total.currency {
                            Currency::MXN => stripe::Currency::MXN,
                            Currency::USD => stripe::Currency::USD,
                            _ => stripe::Currency::USD,
                        },
                        product: part.id.clone(),
                        unit_amount: selected_part_quote.sub_total.amount as u64,
                    },
                    quantity: part.quantity,
                }
            })
            .collect::<Vec<QuoteLineItem>>();

        Ok(quote_line_items)
    }
}
