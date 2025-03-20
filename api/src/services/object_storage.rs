use crate::shared::Result;
use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait ObjectStorage: Send + Sync + 'static {
    async fn put_object_presigned_url(
        &self,
        filepath: &str,
        expires_in: Duration,
    ) -> Result<String>;
    async fn get_object_presigned_url(&self, url: &str, expires_in: Duration) -> Result<String>;
    async fn delete_object(&self, url: &str) -> Result<()>;
    async fn bulk_delete_objects(&self, urls: Vec<&str>) -> Result<()>;
}
