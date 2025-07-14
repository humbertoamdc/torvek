use crate::parts::models::dynamodb_requests::UpdatablePart;
use crate::parts::models::inputs::UpdateSelectedPartQuoteInput;
use crate::parts::models::responses::UpdateSelectedPartQuoteResponse;
use crate::repositories::parts::PartsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdateSelectedPartQuote<P>
where
    P: PartsRepository,
{
    parts_repository: Arc<P>,
}

impl<P> UpdateSelectedPartQuote<P>
where
    P: PartsRepository,
{
    pub fn new(parts_repository: Arc<P>) -> Self {
        Self { parts_repository }
    }
}

#[async_trait]
impl<P> UseCase<UpdateSelectedPartQuoteInput, UpdateSelectedPartQuoteResponse>
    for UpdateSelectedPartQuote<P>
where
    P: PartsRepository,
{
    async fn execute(
        &self,
        input: UpdateSelectedPartQuoteInput,
    ) -> Result<UpdateSelectedPartQuoteResponse> {
        // Update selected part quote id in part.
        let mut updatable_part =
            UpdatablePart::partial_new(input.identity.id, input.part_id.clone());
        updatable_part.selected_part_quote_id = Some(input.selected_part_quote_id.clone());

        let part = self.parts_repository.update(updatable_part).await?;

        Ok(UpdateSelectedPartQuoteResponse { part })
    }
}
