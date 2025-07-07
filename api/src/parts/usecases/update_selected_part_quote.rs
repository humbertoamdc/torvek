use crate::parts::models::dynamodb_requests::UpdatablePart;
use crate::parts::models::inputs::UpdateSelectedPartQuoteInput;
use crate::parts::models::responses::UpdateSelectedPartQuoteResponse;
use crate::repositories::parts::PartsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
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
impl UseCase<UpdateSelectedPartQuoteInput, UpdateSelectedPartQuoteResponse>
    for UpdateSelectedPartQuoteUseCase
{
    async fn execute(
        &self,
        input: UpdateSelectedPartQuoteInput,
    ) -> Result<UpdateSelectedPartQuoteResponse> {
        // Update selected part quote id in part.
        let mut updatable_part =
            UpdatablePart::partial_new(input.quotation_id.clone(), input.part_id.clone());
        updatable_part.selected_part_quote_id = Some(input.selected_part_quote_id.clone());

        let part = self.parts_repository.update_part(updatable_part).await?;

        Ok(UpdateSelectedPartQuoteResponse { part })
    }
}
