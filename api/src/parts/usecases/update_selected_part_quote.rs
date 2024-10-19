use crate::parts::domain::updatable_part::UpdatablePart;
use crate::parts::repositories::parts::PartsRepository;
use crate::shared::usecase::UseCase;
use api_boundary::common::money::Money;
use api_boundary::parts::errors::PartsError;
use api_boundary::parts::models::{Part, PartQuote};
use api_boundary::parts::requests::UpdateSelectedPartQuoteRequest;
use api_boundary::parts::responses::UpdateSelectedPartQuoteResponse;
use axum::async_trait;
use std::sync::Arc;

pub struct UpdateSelectedPartQuoteUseCase {
    parts_repository: Arc<dyn PartsRepository>,
}

impl UpdateSelectedPartQuoteUseCase {
    pub fn new(parts_repository: Arc<dyn PartsRepository>) -> Self {
        Self { parts_repository }
    }
}

#[async_trait]
impl UseCase<UpdateSelectedPartQuoteRequest, UpdateSelectedPartQuoteResponse, PartsError>
    for UpdateSelectedPartQuoteUseCase
{
    async fn execute(
        &self,
        request: UpdateSelectedPartQuoteRequest,
    ) -> Result<UpdateSelectedPartQuoteResponse, PartsError> {
        // Update selected part quote id in part.
        let mut updatable_part =
            UpdatablePart::partial_new(request.quotation_id.clone(), request.part_id.clone());
        updatable_part.selected_part_quote_id = Some(request.selected_part_quote_id.clone());

        let part = self.parts_repository.update_part(updatable_part).await?;

        // Fetch parts for quotation.
        let mut parts_for_quotation = self
            .parts_repository
            .query_parts_for_quotation(request.quotation_id)
            .await?;

        // At this point it is possible that we get the part values before being updated
        // because of eventual consistency. To make sure we have the most up-to-date value
        // use the part returned by the `update_part` method.
        let old_part = parts_for_quotation
            .iter_mut()
            .find(|old_part| old_part.id == part.id)
            .expect("expecting to find a matching part");
        let _ = std::mem::replace(old_part, part.clone());

        // Calculate quotation subtotal.
        let quotation_subtotal = self.calculate_quotation_subtotal(parts_for_quotation.clone());

        Ok(UpdateSelectedPartQuoteResponse {
            part,
            quotation_subtotal,
        })
    }
}

impl UpdateSelectedPartQuoteUseCase {
    pub fn calculate_quotation_subtotal(&self, parts: Vec<Part>) -> Money {
        let selected_part_quotes = parts
            .into_iter()
            .map(|part| {
                part.part_quotes
                    .unwrap()
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
