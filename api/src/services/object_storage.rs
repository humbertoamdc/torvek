use std::time::Duration;

use crate::shared::Result;
use axum::async_trait;

#[async_trait]
pub trait ObjectStorage: Send + Sync + 'static {
    async fn put_object_presigned_url(
        &self,
        filepath: &str,
        expires_in: Duration,
    ) -> Result<String>;
    async fn get_object_presigned_url(&self, url: &str, expires_in: Duration) -> Result<String>;
}
