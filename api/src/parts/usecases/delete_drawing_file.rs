use crate::auth::models::session::Identity;
use crate::parts::models::part::Part;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;
use crate::shared::{PartId, ProjectId, QuoteId, Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct DeleteDrawingFileInput {
    pub customer: Identity,
    pub project_id: ProjectId,
    pub quote_id: QuoteId,
    pub part_id: PartId,
}

pub struct DeleteDrawingFile<P, Q>
where
    P: PartsRepository,
    Q: QuotesRepository,
{
    parts_repository: Arc<P>,
    quotes_repository: Arc<Q>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl<P, Q> DeleteDrawingFile<P, Q>
where
    P: PartsRepository,
    Q: QuotesRepository,
{
    pub fn new(
        parts_repository: Arc<P>,
        quotes_repository: Arc<Q>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            parts_repository,
            quotes_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl<P, Q> UseCase<DeleteDrawingFileInput, Part> for DeleteDrawingFile<P, Q>
where
    P: PartsRepository,
    Q: QuotesRepository,
{
    async fn execute(&self, input: DeleteDrawingFileInput) -> Result<Part> {
        let quotation = self
            .quotes_repository
            .get(input.customer.id.clone(), input.quote_id.clone())
            .await?;

        // Check that the quotation is in an updatable status and change status to created after making an update.
        match quotation.status {
            QuoteStatus::Created => (),
            QuoteStatus::PendingReview | QuoteStatus::PendingPayment => {
                let _ = self
                    .quotes_repository
                    .update_status(
                        input.customer.id.clone(),
                        input.project_id.clone(),
                        input.quote_id.clone(),
                        Some(QuoteStatus::Created),
                    )
                    .await?;
            }
            QuoteStatus::Payed => return Err(Error::UpdatePartAfterPayingQuotation),
        }

        let part = self
            .parts_repository
            .delete_drawing_file(input.customer.id.clone(), input.part_id)
            .await?;

        if let Some(drawing_file) = &part.drawing_file {
            let _ = self
                .object_storage
                .delete_object(&drawing_file.key)
                .await
                .map_err(|err| {
                    tracing::error!(
                        "Failed to delete drawing file for part with id {}: {}",
                        part.id,
                        err
                    );
                    err
                });
        }

        Ok(part)
    }
}
