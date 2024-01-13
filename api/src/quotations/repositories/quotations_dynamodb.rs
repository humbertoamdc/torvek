use crate::quotations::domain::errors::QuotationsError;
use crate::quotations::repositories::quotations::QuotationsRepository;
use api_boundary::quotations::models::Quotation;
use axum::async_trait;
use serde_dynamo::to_item;

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
        let item = to_item(quotation).expect("error converting to dynamodb item");
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
}
