use crate::parts::repositories::parts::PartsRepository;
use crate::parts::services::object_storage::ObjectStorage;
use crate::quotations::usecases::get_quotation_by_id::GetQuotationByIdUseCase;
use crate::shared::{Result, UseCase};
use api_boundary::common::error::Error;
use api_boundary::parts::requests::CreateModelUploadUrlRequest;
use api_boundary::parts::responses::CreateModelUploadUrlResponse;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::GetQuotationByIdRequest;
use axum::async_trait;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

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
impl UseCase<CreateModelUploadUrlRequest, CreateModelUploadUrlResponse> for ModelUploadUrlUseCase {
    async fn execute(
        &self,
        request: CreateModelUploadUrlRequest,
    ) -> Result<CreateModelUploadUrlResponse> {
        if self.quotation_is_payed(&request).await? {
            return Err(Error::UpdatePartAfterPayingQuotation);
        }

        let part = self
            .parts_repository
            .get_part(request.quotation_id, request.part_id)
            .await?;

        let url = part.model_file.url.parse::<Url>().unwrap();
        let file_path = url.path().strip_prefix("/").unwrap().to_string();

        let presigned_url = self
            .object_storage
            .put_object_presigned_url(file_path, Duration::from_secs(300))
            .await?;
        let url = presigned_url.split("?").nth(0).unwrap().to_string();

        Ok(CreateModelUploadUrlResponse { url, presigned_url })
    }
}

impl ModelUploadUrlUseCase {
    async fn quotation_is_payed(&self, request: &CreateModelUploadUrlRequest) -> Result<bool> {
        let get_quotation_request = GetQuotationByIdRequest {
            customer_id: String::default(),
            project_id: request.project_id.clone(),
            quotation_id: request.quotation_id.clone(),
        };
        let quotation = self
            .get_quotation_by_id_use_case
            .execute(get_quotation_request)
            .await
            .map_err(|_| Error::UnknownError)?; // TODO: Handle error properly.

        Ok(quotation.status == QuotationStatus::Payed)
    }
}
