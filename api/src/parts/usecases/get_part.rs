use crate::parts::models::inputs::GetPartInput;
use crate::parts::models::part::Part;
use crate::repositories::parts::PartsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

pub struct GetPart {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl GetPart {
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
impl UseCase<GetPartInput, Part> for GetPart {
    async fn execute(&self, input: GetPartInput) -> Result<Part> {
        let mut part = self
            .parts_repository
            .get(input.identity.id, input.part_id)
            .await?;

        let presigned_url = self
            .object_storage
            .get_object_presigned_url(&part.render_file.url, Duration::from_secs(300))
            .await?;
        part.render_file.presigned_url = Some(presigned_url);

        Ok(part)
    }
}
