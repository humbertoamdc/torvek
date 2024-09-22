use crate::parts::repositories::parts::PartsRepository;
use crate::parts::services::object_storage::ObjectStorage;
use crate::shared::usecase::UseCase;
use api_boundary::parts::errors::PartsError;
use api_boundary::parts::requests::CreateModelUploadUrlRequest;
use api_boundary::parts::responses::CreateModelUploadUrlResponse;
use axum::async_trait;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

pub struct ModelUploadUrlUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl ModelUploadUrlUseCase {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            parts_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl UseCase<CreateModelUploadUrlRequest, CreateModelUploadUrlResponse, PartsError>
    for ModelUploadUrlUseCase
{
    async fn execute(
        &self,
        request: CreateModelUploadUrlRequest,
    ) -> Result<CreateModelUploadUrlResponse, PartsError> {
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
