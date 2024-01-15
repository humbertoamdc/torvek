use crate::parts::domain::errors::PartsError;
use crate::parts::domain::models::DynamodbPartItem;
use crate::parts::repositories::parts::PartsRepository;
use api_boundary::parts::models::Part;
use aws_sdk_dynamodb::types::{PutRequest, WriteRequest};
use axum::async_trait;
use serde_dynamo::to_item;

#[derive(Clone)]
pub struct DynamodbParts {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbParts {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl PartsRepository for DynamodbParts {
    async fn create_parts(&self, parts: Vec<Part>) -> Result<(), PartsError> {
        let items: Vec<WriteRequest> = parts
            .into_iter()
            .map(|part| {
                let part = DynamodbPartItem::from(part);
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(
                                to_item(part).expect("error converting to dynamodb item"),
                            ))
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect();

        let response = self
            .client
            .batch_write_item()
            .request_items(&self.table, items)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(PartsError::PartsBatchCreateError),
        }
    }
}
