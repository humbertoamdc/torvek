use crate::parts::models::inputs::CreateModelUploadUrlInput;
use crate::parts::models::responses::CreateModelUploadUrlResponse;
use crate::quotations::models::inputs::GetQuotationByIdInput;
use crate::quotations::models::quotation::QuotationStatus;
use crate::quotations::usecases::get_quotation_by_id::GetQuotationByIdUseCase;
use crate::repositories::parts::PartsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

static PRESIGNED_URLS_GET_DURATION_SECONDS: u64 = 3600;

pub struct ModelUploadUrlUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
    get_quotation_by_id_use_case: GetQuotationByIdUseCase,
}

impl ModelUploadUrlUseCase {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
        get_quotation_by_id_use_case: GetQuotationByIdUseCase,
    ) -> Self {
        Self {
            parts_repository,
            object_storage,
            get_quotation_by_id_use_case,
        }
    }
}

#[async_trait]
impl UseCase<CreateModelUploadUrlInput, CreateModelUploadUrlResponse> for ModelUploadUrlUseCase {
    async fn execute(
        &self,
        input: CreateModelUploadUrlInput,
    ) -> Result<CreateModelUploadUrlResponse> {
        if self.quotation_is_payed(&input).await? {
            return Err(Error::UpdatePartAfterPayingQuotation);
        }

        let part = self
            .parts_repository
            .get_part(input.quotation_id, input.part_id)
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

impl ModelUploadUrlUseCase {
    async fn quotation_is_payed(&self, input: &CreateModelUploadUrlInput) -> Result<bool> {
        let get_quotation_request = GetQuotationByIdInput {
            identity: input.identity.clone(),
            project_id: input.project_id.clone(),
            quotation_id: input.quotation_id.clone(),
        };
        let quotation = self
            .get_quotation_by_id_use_case
            .execute(get_quotation_request)
            .await
            .map_err(|_| Error::UnknownError)?; // TODO: Handle error properly.

        Ok(quotation.status == QuotationStatus::Payed)
    }
}
