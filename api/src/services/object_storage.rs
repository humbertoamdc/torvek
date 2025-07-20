use crate::shared::Result;
use async_trait::async_trait;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::time::Duration;

#[derive(Serialize_enum_str, Deserialize_enum_str, Debug, Clone)]
pub enum ObjectStorageOperation {
    Get,
    Put,
}

#[async_trait]
pub trait ObjectStorage: Send + Sync + 'static {
    fn endpoint_url(&self) -> String;
    async fn put_object_presigned_url(&self, key: &str, expires_in: Duration) -> Result<String>;
    async fn get_object_presigned_url(&self, key: &str, expires_in: Duration) -> Result<String>;
    async fn delete_object(&self, key: &str) -> Result<()>;
    async fn bulk_delete_objects(&self, keys: Vec<&str>) -> Result<()>;
}
