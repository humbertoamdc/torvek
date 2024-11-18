use crate::parts::domain::dynamodb_requests::UpdatablePart;
use crate::repositories::parts::PartsRepository;
use crate::shared::{Result, UseCase};
use api_boundary::common::money::Money;
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
impl UseCase<UpdateSelectedPartQuoteRequest, UpdateSelectedPartQuoteResponse>
    for UpdateSelectedPartQuoteUseCase
{
    async fn execute(
        &self,
        request: UpdateSelectedPartQuoteRequest,
    ) -> Result<UpdateSelectedPartQuoteResponse> {
        // Update selected part quote id in part.
        let mut updatable_part =
            UpdatablePart::partial_new(request.quotation_id.clone(), request.part_id.clone());
        updatable_part.selected_part_quote_id = Some(request.selected_part_quote_id.clone());

        let part = self.parts_repository.update_part(updatable_part).await?;

        Ok(UpdateSelectedPartQuoteResponse { part })
    }
}
