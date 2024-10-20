use crate::parts::repositories::parts::PartsRepository;
use crate::parts::services::object_storage::ObjectStorage;
use crate::shared::{Result, UseCase};
use api_boundary::parts::models::Part;
use api_boundary::parts::requests::GetPartRequest;
use axum::async_trait;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

pub struct GetPartUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl GetPartUseCase {
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
impl UseCase<GetPartRequest, Part> for GetPartUseCase {
    async fn execute(&self, request: GetPartRequest) -> Result<Part> {
        let mut part = self
            .parts_repository
            .get_part(request.quotation_id, request.part_id)
            .await?;

        let url = part.render_file.url.parse::<Url>().unwrap();
        let file_path = url.path().strip_prefix("/").unwrap().to_string();

        let presigned_url = self
            .object_storage
            .get_object_presigned_url(file_path, Duration::from_secs(300))
            .await?;
        part.render_file.presigned_url = Some(presigned_url);

        Ok(part)
    }
}
