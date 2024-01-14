use std::time::Duration;

use crate::parts::domain::errors::PartsError;
use crate::parts::services::object_storage::ObjectStorage;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::async_trait;

#[derive(Clone)]
pub struct S3ObjectStorage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3ObjectStorage {
    pub fn new(client: aws_sdk_s3::Client, bucket: String) -> Self {
        Self { client, bucket }
    }
}

#[async_trait]
impl ObjectStorage for S3ObjectStorage {
    async fn put_object_presigned_url(
        &self,
        file_path: String,
        expires_in: Duration,
    ) -> Result<String, PartsError> {
        let result = self
            .client
            .put_object()
            .bucket(self.bucket.clone())
            .key(file_path)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await;

        match result {
            Ok(presigned_url) => Ok(presigned_url.uri().to_string()),
            Err(_) => Err(PartsError::PresignedUrlGenerationError),
        }
    }
}
