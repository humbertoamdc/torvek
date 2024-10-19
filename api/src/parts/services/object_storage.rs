use std::time::Duration;

use api_boundary::common::error::Error;
use axum::async_trait;

#[async_trait]
pub trait ObjectStorage: Send + Sync + 'static {
    async fn put_object_presigned_url(
        &self,
        file_path: String,
        expires_in: Duration,
    ) -> Result<String, Error>;
    async fn get_object_presigned_url(
        &self,
        file_path: String,
        expires_in: Duration,
    ) -> Result<String, Error>;
}
