use std::time::Duration;

use crate::shared::Result;
use api_boundary::common::error::Error;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::types::builders::DeleteBuilder;
use aws_sdk_s3::types::ObjectIdentifier;
use axum::async_trait;
use url::Url;

use crate::services::object_storage::ObjectStorage;

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
        filepath: &str,
        expires_in: Duration,
    ) -> Result<String> {
        let result = self
            .client
            .put_object()
            .bucket(self.bucket.clone())
            .key(filepath)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await;

        match result {
            Ok(presigned_url) => Ok(presigned_url.uri().to_string()),
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn get_object_presigned_url(&self, url: &str, expires_in: Duration) -> Result<String> {
        let result = self
            .client
            .get_object()
            .bucket(self.bucket.clone())
            .key(self.filepath(url)?)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await;

        match result {
            Ok(presigned_url) => Ok(presigned_url.uri().to_string()),
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn delete_object(&self, url: &str) -> Result<()> {
        let result = self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(&self.filepath(url)?)
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

    async fn bulk_delete_objects(&self, urls: Vec<&str>) -> Result<()> {
        let object_identifiers = urls
            .into_iter()
            .map(|url| {
                ObjectIdentifier::builder()
                    .key(self.filepath(url).unwrap())
                    .build()
                    .unwrap()
            })
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

impl S3ObjectStorage {
    fn filepath(&self, url: &str) -> Result<String> {
        match Url::parse(url) {
            Ok(parsed_url) => match parsed_url.path().strip_prefix("/") {
                Some(filepath) => Ok(filepath.to_string()),
                None => Err(Error::UnknownError),
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}
