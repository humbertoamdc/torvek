use aws_sdk_dynamodb::types::AttributeValue;
use axum::async_trait;
use serde_dynamo::from_items;

use api_boundary::parts::models::PartQuote;

use crate::parts::domain::errors::PartsError;
use crate::parts::repositories::part_quotes::PartQuotesRepository;

pub struct DynamodbPartQuotes {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbPartQuotes {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl PartQuotesRepository for DynamodbPartQuotes {
    async fn query_part_quotes_for_part(
        &self,
        part_id: String,
    ) -> Result<Vec<PartQuote>, PartsError> {
        let response = self
            .client
            .query()
            .key_condition_expression("part_id = :part_id")
            .expression_attribute_values(":part_id", AttributeValue::S(part_id))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(part_quotes) => Ok(part_quotes),
                    Err(err) => {
                        log::error!("{err:?}");
                        Err(PartsError::UnknownError)
                    }
                }
            }
            Err(err) => {
                log::error!("{err:?}");
                Err(PartsError::QueryPartQuotesError)
            }
        }
    }
}
