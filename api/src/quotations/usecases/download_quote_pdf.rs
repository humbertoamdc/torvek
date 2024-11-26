use crate::auth::application::services::identity_manager::IdentityManager;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::services::payment_processor::{PaymentsProcessor, PriceData, QuoteLineItem};
use crate::shared;
use crate::shared::UseCase;
use api_boundary::common::error::Error;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::DownloadQuotePdfRequest;
use axum::async_trait;
use axum::body::Bytes;
use iso_currency::Currency;
use shared::Result;
use std::sync::Arc;

pub struct DownloadQuotePdfUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
    payments_processor: Arc<dyn PaymentsProcessor>,
    identity_manager: Arc<dyn IdentityManager>,
}

impl DownloadQuotePdfUseCase {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        quotations_repository: Arc<dyn QuotationsRepository>,
        payments_processor: Arc<dyn PaymentsProcessor>,
        identity_manager: Arc<dyn IdentityManager>,
    ) -> Self {
        Self {
            parts_repository,
            quotations_repository,
            payments_processor,
            identity_manager,
        }
    }
}

#[async_trait]
impl UseCase<DownloadQuotePdfRequest, Bytes> for DownloadQuotePdfUseCase {
    async fn execute(&self, request: DownloadQuotePdfRequest) -> crate::shared::Result<Bytes> {
        // TODO: Move auth errors to common error enum.
        let session = self
            .identity_manager
            .get_session(request.session_id.clone())
            .await
            .map_err(|_| Error::UnknownError)?;

        let identity = self
            .identity_manager
            .get_identity(session.identity.clone().id)
            .await
            .map_err(|_| Error::UnknownError)?;

        tracing::info!("{session:#?}");

        self.check_quote_status(request.project_id, request.quotation_id.clone())
            .await?;

        let quote_line_items = self.generate_quote_line_items(request.quotation_id).await?;

        let stripe_customer_id = identity
            .metadata_admin
            .expect(&format!(
                "missing admin metadata for customer with id: {}",
                session.identity.id
            ))
            .stripe_customer_id;

        let stripe_quote = self
            .payments_processor
            .create_quote(stripe_customer_id, quote_line_items)
            .await?;

        self.payments_processor
            .finalize_quote(stripe_quote.id.clone())
            .await?;

        self.payments_processor
            .download_quote_pdf(stripe_quote.id)
            .await
    }
}

impl DownloadQuotePdfUseCase {
    async fn check_quote_status(&self, project_id: String, quotation_id: String) -> Result<()> {
        let quote = self
            .quotations_repository
            .get_quotation_by_id(project_id, quotation_id.clone())
            .await?;

        match quote.status {
            QuotationStatus::PendingPayment | QuotationStatus::Payed => Ok(()),
            _ => Err(Error::NoPdfQuoteAvailable),
        }
    }

    async fn generate_quote_line_items(&self, quotation_id: String) -> Result<Vec<QuoteLineItem>> {
        let page_limit = 100;
        let query_part_response = self
            .parts_repository
            .query_parts_for_quotation(quotation_id, page_limit, None)
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
                        product: selected_part_quote.part_id.clone(),
                        unit_amount: selected_part_quote.sub_total.amount as u64,
                    },
                    quantity: part.quantity,
                }
            })
            .collect::<Vec<QuoteLineItem>>();

        Ok(quote_line_items)
    }
}
