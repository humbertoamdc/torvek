use std::time::Duration;

use crate::parts::domain::errors::PartsError;
use axum::async_trait;

#[async_trait]
pub trait ObjectStorage: Send + Sync + 'static {
    async fn put_object_presigned_url(
        &self,
        file_path: String,
        expires_in: Duration,
    ) -> Result<String, PartsError>;
}
