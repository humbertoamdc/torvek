use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::Result;
use crate::shared::UseCase;
use api_boundary::common::error::Error;
use api_boundary::parts::requests::DeletePartRequest;
use api_boundary::quotations::models::QuotationStatus;
use axum::async_trait;
use std::sync::Arc;

pub struct DeletePartUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl DeletePartUseCase {
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
impl UseCase<DeletePartRequest, ()> for DeletePartUseCase {
    async fn execute(&self, request: DeletePartRequest) -> Result<()> {
        let quotation = self
            .quotations_repository
            .get_quotation_by_id(request.project_id, request.quotation_id.clone())
            .await?;

        // Check that the quotation is in an updatable status.
        match quotation.status {
            QuotationStatus::Payed => return Err(Error::UpdatePartAfterPayingQuotation),
            _ => (),
        }

        let part = self
            .parts_repository
            .delete_part(request.quotation_id, request.part_id)
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
