use crate::shared::Result;
use async_trait::async_trait;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::types::builders::DeleteBuilder;
use aws_sdk_s3::types::ObjectIdentifier;
use std::time::Duration;

use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;

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
    async fn put_object_presigned_url(&self, key: &str, expires_in: Duration) -> Result<String> {
        let result = self
            .client
            .put_object()
            .bucket(self.bucket.clone())
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await;

        match result {
            Ok(presigned_url) => Ok(presigned_url.uri().to_string()),
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn get_object_presigned_url(&self, key: &str, expires_in: Duration) -> Result<String> {
        let result = self
            .client
            .get_object()
            .bucket(self.bucket.clone())
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await;

        match result {
            Ok(presigned_url) => Ok(presigned_url.uri().to_string()),
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn delete_object(&self, key: &str) -> Result<()> {
        let result = self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn bulk_delete_objects(&self, keys: Vec<&str>) -> Result<()> {
        let object_identifiers = keys
            .into_iter()
            .map(|key| ObjectIdentifier::builder().key(key).build().unwrap())
            .collect::<Vec<ObjectIdentifier>>();

        if object_identifiers.is_empty() {
            return Ok(());
        }

        let result = self
            .client
            .delete_objects()
            .bucket(&self.bucket)
            .delete(
                DeleteBuilder::default()
                    .set_objects(Some(object_identifiers))
                    .build()
                    .unwrap(),
            )
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}
