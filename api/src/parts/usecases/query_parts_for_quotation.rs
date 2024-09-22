use std::sync::Arc;
use std::time::Duration;

use axum::async_trait;

use api_boundary::common::money::Money;
use api_boundary::parts::errors::PartsError;
use api_boundary::parts::models::{Part, PartQuote};
use api_boundary::parts::requests::QueryPartsForQuotationRequest;
use api_boundary::parts::responses::QueryPartsForQuotationResponse;
use url::Url;

use crate::parts::repositories::parts::PartsRepository;
use crate::parts::services::object_storage::ObjectStorage;
use crate::shared::usecase::UseCase;

static PRESIGNED_URLS_GET_DURATION_SECONDS: u64 = 3600;

pub struct QueryPartsForQuotationUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl QueryPartsForQuotationUseCase {
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
impl UseCase<QueryPartsForQuotationRequest, QueryPartsForQuotationResponse, PartsError>
    for QueryPartsForQuotationUseCase
{
    async fn execute(
        &self,
        request: QueryPartsForQuotationRequest,
    ) -> Result<QueryPartsForQuotationResponse, PartsError> {
        let mut parts = self
            .parts_repository
            .query_parts_for_quotation(request.quotation_id)
            .await?;

        self.sign_part_render_urls(&mut parts).await?;

        let quotation_subtotal: Option<Money> = {
            match request.with_quotation_subtotal {
                true => match self.get_quotation_subtotal(&parts).await {
                    Ok(money) => Ok(Some(money)),
                    Err(err) => match err {
                        PartsError::NoSelectedQuoteAvailableForPart(_) => Ok(None),
                        _ => Err(err),
                    },
                },
                false => Ok(None),
            }?
        };

        Ok(QueryPartsForQuotationResponse {
            parts,
            quotation_subtotal,
        })
    }
}

impl QueryPartsForQuotationUseCase {
    async fn sign_part_render_urls(&self, parts: &mut Vec<Part>) -> Result<(), PartsError> {
        for part in parts.iter_mut() {
            // TODO: Handle error.
            // For both cases, we can recover from the error, we will just leave the presigned url field
            // as none. In the future we might page to have someone have a look at the url format.
            let part_render_url =
                Url::parse(&part.render_file.url).expect("invalid render file url");
            let part_render_url_path = part_render_url
                .path()
                .strip_prefix("/")
                .unwrap()
                .to_string();
            let presigned_url = self
                .object_storage
                .get_object_presigned_url(
                    part_render_url_path,
                    Duration::from_secs(PRESIGNED_URLS_GET_DURATION_SECONDS),
                )
                .await?;

            part.render_file.presigned_url = Some(presigned_url);
        }

        Ok(())
    }

    async fn get_quotation_subtotal(&self, parts: &Vec<Part>) -> Result<Money, PartsError> {
        let selected_part_quotes = parts
            .iter()
            .map(|part| part.part_quotes.clone().unwrap_or_default())
            .flatten()
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
