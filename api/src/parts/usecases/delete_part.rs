use crate::parts::models::inputs::DeletePartInput;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;
use crate::shared::Result;
use crate::shared::UseCase;
use async_trait::async_trait;
use std::sync::Arc;

pub struct DeletePart<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    parts_repository: Arc<P>,
    quotations_repository: Arc<Q>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl<Q, P> DeletePart<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    pub fn new(
        parts_repository: Arc<P>,
        quotations_repository: Arc<Q>,
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
impl<Q, P> UseCase<DeletePartInput, ()> for DeletePart<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    async fn execute(&self, input: DeletePartInput) -> Result<()> {
        let quotation = self
            .quotations_repository
            .get(input.identity.id.clone(), input.quotation_id.clone())
            .await?;

        // Check that the quotation is in an updatable status.
        match quotation.status {
            QuoteStatus::Payed => return Err(Error::UpdatePartAfterPayingQuotation),
            _ => (),
        }

        let part = self
            .parts_repository
            .delete(input.identity.id, input.part_id)
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
