use crate::auth::models::session::Identity;
use crate::services::object_storage::{ObjectStorage, ObjectStorageOperation};
use crate::shared::error::Error;
use crate::shared::UseCase;
use async_trait::async_trait;
use http::Uri;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

static PRESIGNED_URL_DURATION_SECONDS: u64 = 300;

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneratePresignedUrlInput {
    pub identity: Identity,
    pub key: String,
    pub operation: ObjectStorageOperation,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneratePresignedUrlResponse {
    pub presigned_url: String,
}

pub struct GeneratePresignedUrl {
    object_storage: Arc<dyn ObjectStorage>,
}

impl GeneratePresignedUrl {
    pub fn new(object_storage: Arc<dyn ObjectStorage>) -> Self {
        Self { object_storage }
    }
}

#[async_trait]
impl UseCase<GeneratePresignedUrlInput, GeneratePresignedUrlResponse> for GeneratePresignedUrl {
    async fn execute(
        &self,
        input: GeneratePresignedUrlInput,
    ) -> crate::shared::Result<GeneratePresignedUrlResponse> {
        if !input.key.contains(&input.identity.id) {
            return Err(Error::Forbidden);
        }

        let presigned_url = match input.operation {
            ObjectStorageOperation::Get => {
                self.object_storage
                    .get_object_presigned_url(
                        &input.key,
                        Duration::from_secs(PRESIGNED_URL_DURATION_SECONDS),
                    )
                    .await
            }
            ObjectStorageOperation::Put => {
                self.object_storage
                    .put_object_presigned_url(
                        &input.key,
                        Duration::from_secs(PRESIGNED_URL_DURATION_SECONDS),
                    )
                    .await
            }
        }?;

        Ok(GeneratePresignedUrlResponse { presigned_url })
    }
}
