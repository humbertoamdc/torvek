use crate::services::object_storage::{ObjectStorage, ObjectStorageOperation};
use crate::shared::error::Error;
use crate::shared::UseCase;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

static PRESIGNED_URL_DURATION_SECONDS: u64 = 300;

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminGeneratePresignedUrlInput {
    pub key: String,
    pub operation: ObjectStorageOperation,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminGeneratePresignedUrlResponse {
    pub presigned_url: String,
}

pub struct AdminGeneratePresignedUrl {
    pub object_storage: Arc<dyn ObjectStorage>,
}

impl AdminGeneratePresignedUrl {
    pub fn new(object_storage: Arc<dyn ObjectStorage>) -> Self {
        Self { object_storage }
    }
}

#[async_trait]
impl UseCase<AdminGeneratePresignedUrlInput, AdminGeneratePresignedUrlResponse>
    for AdminGeneratePresignedUrl
{
    async fn execute(
        &self,
        input: AdminGeneratePresignedUrlInput,
    ) -> crate::shared::Result<AdminGeneratePresignedUrlResponse> {
        let presigned_url = match input.operation {
            ObjectStorageOperation::Get => {
                self.object_storage
                    .get_object_presigned_url(
                        &input.key,
                        Duration::from_secs(PRESIGNED_URL_DURATION_SECONDS),
                    )
                    .await
            }
            ObjectStorageOperation::Put => Err(Error::Forbidden),
        }?;

        Ok(AdminGeneratePresignedUrlResponse { presigned_url })
    }
}
