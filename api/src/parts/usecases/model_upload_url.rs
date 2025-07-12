use crate::parts::models::inputs::CreateModelUploadUrlInput;
use crate::parts::models::responses::CreateModelUploadUrlResponse;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

static PRESIGNED_URLS_GET_DURATION_SECONDS: u64 = 3600;

pub struct ModelUploadUrl {
    parts_repository: Arc<dyn PartsRepository>,
    quotations_repository: Arc<dyn QuotesRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl ModelUploadUrl {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        quotations_repository: Arc<dyn QuotesRepository>,
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
impl UseCase<CreateModelUploadUrlInput, CreateModelUploadUrlResponse> for ModelUploadUrl {
    async fn execute(
        &self,
        input: CreateModelUploadUrlInput,
    ) -> Result<CreateModelUploadUrlResponse> {
        if self.quotation_is_payed(&input).await? {
            return Err(Error::UpdatePartAfterPayingQuotation);
        }

        let part = self
            .parts_repository
            .get(input.identity.id, input.part_id)
            .await?;

        let url = part.model_file.url.parse::<Url>().unwrap();
        let filepath = url.path().strip_prefix("/").unwrap();

        let presigned_url = self
            .object_storage
            .put_object_presigned_url(
                filepath,
                Duration::from_secs(PRESIGNED_URLS_GET_DURATION_SECONDS),
            )
            .await?;
        let url = presigned_url.split("?").nth(0).unwrap().to_string();

        Ok(CreateModelUploadUrlResponse { url, presigned_url })
    }
}

impl ModelUploadUrl {
    async fn quotation_is_payed(&self, input: &CreateModelUploadUrlInput) -> Result<bool> {
        let quotation = self
            .quotations_repository
            .get(input.project_id.clone(), input.quotation_id.clone())
            .await?;

        Ok(quotation.status == QuoteStatus::Payed)
    }
}
