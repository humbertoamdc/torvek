use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::domain::models::DynamodbQuotationItem;
use crate::quotations::repositories::quotations::QuotationsRepository;
use api_boundary::quotations::models::Quotation;
use aws_sdk_dynamodb::types::AttributeValue;
use axum::async_trait;
use serde_dynamo::{from_items, to_item};

#[derive(Clone)]
pub struct DynamodbQuotations {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbQuotations {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl QuotationsRepository for DynamodbQuotations {
    async fn create_quotation(&self, quotation: Quotation) -> Result<(), QuotationsError> {
        let dynamodb_quotation = DynamodbQuotationItem::from(quotation);
        let item = to_item(dynamodb_quotation).expect("error converting to dynamodb item");
        let response = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(QuotationsError::CreateQuotationError),
        }
    }

    async fn query_quotations_for_project(
        &self,
        client_id: String,
        project_id: String,
    ) -> Result<Vec<Quotation>, QuotationsError> {
        let client_id_and_project_id = format!("{client_id}#{project_id}");

        // TODO: Get ordered by date.
        let response = self
            .client
            .query()
            .key_condition_expression("#client_id_project_id = :value")
            .expression_attribute_values(":value", AttributeValue::S(client_id_and_project_id))
            .expression_attribute_names("#client_id_project_id", "client_id#project_id")
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_quotations) => {
                        let quotations = dynamodb_quotations
                            .into_iter()
                            .map(|q: DynamodbQuotationItem| q.into())
                            .collect();

                        Ok(quotations)
                    }
                    Err(_) => Err(QuotationsError::UnknownError),
                }
            }
            Err(_) => Err(QuotationsError::QueryQuotationsError),
        }
    }
}
