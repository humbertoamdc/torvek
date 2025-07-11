use crate::parts::models::inputs::QueryPartsForQuotationInput;
use crate::parts::models::part::{Part, PartQuote};
use crate::parts::models::responses::QueryPartsForQuotationResponse;
use crate::repositories::parts::PartsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;
use crate::shared::money::Money;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

static PRESIGNED_URLS_GET_DURATION_SECONDS: u64 = 3600;

pub struct QueryPartsByQuotation {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl QueryPartsByQuotation {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            parts_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl UseCase<QueryPartsForQuotationInput, QueryPartsForQuotationResponse>
    for QueryPartsByQuotation
{
    async fn execute(
        &self,
        input: QueryPartsForQuotationInput,
    ) -> Result<QueryPartsForQuotationResponse> {
        let mut response = self
            .parts_repository
            .query(
                input.identity.id,
                input.quotation_id,
                input.cursor,
                input.limit,
            )
            .await?;

        self.sign_part_render_urls(&mut response.data).await?;

        let quotation_subtotal: Option<Money> = {
            match input.with_quotation_subtotal {
                true => match self.get_quotation_subtotal(&response.data).await {
                    Ok(money) => Ok(Some(money)),
                    Err(err) => match err {
                        Error::NoSelectedQuoteAvailableForPart(_) => Ok(None),
                        _ => Err(err),
                    },
                },
                false => Ok(None),
            }?
        };

        Ok(QueryPartsForQuotationResponse {
            parts: response.data,
            quotation_subtotal,
            cursor: response.cursor,
        })
    }
}

impl QueryPartsByQuotation {
    async fn sign_part_render_urls(&self, parts: &mut Vec<Part>) -> Result<()> {
        for part in parts.iter_mut() {
            let presigned_url = self
                .object_storage
                .get_object_presigned_url(
                    &part.render_file.url,
                    Duration::from_secs(PRESIGNED_URLS_GET_DURATION_SECONDS),
                )
                .await?;

            part.render_file.presigned_url = Some(presigned_url);
        }

        Ok(())
    }

    async fn get_quotation_subtotal(&self, parts: &Vec<Part>) -> Result<Money> {
        // TODO: Query quotation and only get the subtotal is quotations is in the right status.
        match parts
            .iter()
            .find(|part| part.selected_part_quote_id.is_none())
        {
            Some(part) => return Err(Error::NoSelectedQuoteAvailableForPart(part.id.clone())),
            None => (),
        }

        let selected_part_quotes: Vec<PartQuote> = parts
            .iter()
            .filter_map(|part| {
                part.part_quotes
                    .clone()
                    .expect("expecting part quote")
                    .into_iter()
                    .find(|part_quote| {
                        part_quote.id
                            == part
                                .selected_part_quote_id
                                .clone()
                                .expect("expecting a selected part quote")
                    })
            })
            .collect::<Vec<PartQuote>>();

        let subtotal =
            selected_part_quotes
                .iter()
                .fold(Money::default(), |mut money: Money, part_quote| {
                    money.amount += part_quote.sub_total.amount;
                    money
                });

        Ok(subtotal)
    }
}
