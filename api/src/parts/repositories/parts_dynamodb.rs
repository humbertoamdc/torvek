use crate::parts::domain::errors::PartsError;
use crate::parts::domain::models::DynamodbPartItem;
use crate::parts::repositories::parts::PartsRepository;
use api_boundary::parts::models::Part;
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
use axum::async_trait;
use serde_dynamo::{from_items, to_item};

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

    async fn query_parts_for_quotation(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> Result<Vec<Part>, PartsError> {
        let client_project_and_quotation_ids = format!("{client_id}#{project_id}#{quotation_id}");

        // TODO: Get ordered by date.
        let response = self
            .client
            .query()
            .key_condition_expression("#client_project_and_quotation_ids = :value")
            .expression_attribute_values(
                ":value",
                AttributeValue::S(client_project_and_quotation_ids),
            )
            .expression_attribute_names(
                "#client_project_and_quotation_ids",
                "client_id#project_id#quotation_id",
            )
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_parts) => {
                        let parts = dynamodb_parts
                            .into_iter()
                            .map(|p: DynamodbPartItem| p.into())
                            .collect();

                        Ok(parts)
                    }
                    Err(_) => Err(PartsError::UnknownError),
                }
            }
            Err(_) => Err(PartsError::QueryPartsError),
        }
    }
}
