use std::time::Duration;

use axum::async_trait;

use crate::parts::domain::errors::PartsError;

#[async_trait]
pub trait ObjectStorage: Send + Sync + 'static {
    async fn put_object_presigned_url(
        &self,
        file_path: String,
        expires_in: Duration,
    ) -> Result<String, PartsError>;
    async fn get_object_presigned_url(
        &self,
        file_path: String,
        expires_in: Duration,
    ) -> Result<String, PartsError>;
}
