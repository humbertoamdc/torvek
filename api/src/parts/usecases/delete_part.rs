use crate::parts::models::inputs::DeletePartInput;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;
use crate::shared::Result;
use crate::shared::UseCase;
use async_trait::async_trait;
use std::sync::Arc;

pub struct DeletePart {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl DeletePart {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        quotations_repository: Arc<dyn QuotationsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            parts_repository,
            quotations_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl UseCase<DeletePartInput, ()> for DeletePart {
    async fn execute(&self, input: DeletePartInput) -> Result<()> {
        let quotation = self
            .quotations_repository
            .get(input.project_id, input.quotation_id.clone())
            .await?;

        // Check that the quotation is in an updatable status.
        match quotation.status {
            QuoteStatus::Payed => return Err(Error::UpdatePartAfterPayingQuotation),
            _ => (),
        }

        let part = self
            .parts_repository
            .delete(input.quotation_id, input.part_id)
            .await?;

        let _ = self
            .object_storage
            .delete_object(&part.model_file.url)
            .await;
        let _ = self
            .object_storage
            .delete_object(&part.render_file.url)
            .await;
        if let Some(drawing_file) = part.drawing_file {
            let _ = self.object_storage.delete_object(&drawing_file.url).await;
        }

        Ok(())
    }
}
