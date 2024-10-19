use std::collections::HashMap;

use api_boundary::parts::errors::PartsError;
use aws_sdk_dynamodb::types::{AttributeValue, TransactWriteItem, Update};
use axum::async_trait;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;

use api_boundary::parts::models::PartQuote;
use api_boundary::quotations::models::QuotationStatus;

use crate::parts::services::part_quotes_creation::PartQuotesCreation;

pub struct DynamodbParQuotesCreation {
    client: aws_sdk_dynamodb::Client,
    parts_table: String,
    quotations_table: String,
}

impl DynamodbParQuotesCreation {
    pub fn new(
        client: aws_sdk_dynamodb::Client,
        parts_table: String,
        quotations_table: String,
    ) -> Self {
        Self {
            client,
            parts_table,
            quotations_table,
        }
    }
}

#[async_trait]
impl PartQuotesCreation for DynamodbParQuotesCreation {
    async fn create_part_quotes_and_update_quotation_status(
        &self,
        project_id: String,
        quotation_id: String,
        part_quotes_by_part: HashMap<String, Vec<PartQuote>>,
        selected_part_quote_by_part: HashMap<String, String>,
    ) -> Result<(), PartsError> {
        // Update quotation status to PendingPayment..
        let quotation_transaction =
            self.build_quotation_transaction(project_id, quotation_id.clone());

        // Create part quotes Dynamodb transactions for parts table
        let part_quotes_in_parts_transactions = self.build_part_quotes_transaction_in_parts(
            quotation_id,
            part_quotes_by_part,
            selected_part_quote_by_part,
        );

        // Build transaction request.
        let mut transaction_request = self
            .client
            .transact_write_items()
            .transact_items(quotation_transaction);

        for transaction in part_quotes_in_parts_transactions {
            transaction_request = transaction_request.transact_items(transaction.clone());
        }

        let response = transaction_request.send().await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => {
                log::error!("{:?}", error);
                Err(PartsError::UnknownError)
            }
        }
    }
}

impl DynamodbParQuotesCreation {
    fn build_quotation_transaction(
        &self,
        project_id: String,
        quotation_id: String,
    ) -> TransactWriteItem {
        TransactWriteItem::builder()
            .update(
                Update::builder()
                    .table_name(&self.quotations_table)
                    .set_key(Some(HashMap::from([
                        (String::from("project_id"), AttributeValue::S(project_id)),
                        (String::from("id"), AttributeValue::S(quotation_id)),
                    ])))
                    .update_expression(
                        "SET #status = :pendingPaymentStatus, updated_at = :updated_at",
                    )
                    .set_expression_attribute_names(Some(HashMap::from([(
                        String::from("#status"),
                        String::from("status"),
                    )])))
                    .set_expression_attribute_values(Some(HashMap::from([
                        (
                            String::from(":pendingPaymentStatus"),
                            AttributeValue::S(QuotationStatus::PendingPayment.to_string()),
                        ),
                        (
                            String::from(":updated_at"),
                            AttributeValue::S(chrono::Utc::now().to_rfc3339()),
                        ),
                    ])))
                    .build()
                    .unwrap(),
            )
            .build()
    }

    fn build_part_quotes_transaction_in_parts(
        &self,
        quotation_id: String,
        part_quotes_by_part_map: HashMap<String, Vec<PartQuote>>,
        selected_part_quote_by_part: HashMap<String, String>,
    ) -> Vec<TransactWriteItem> {
        part_quotes_by_part_map
            .into_iter()
            .map(|(part_id, part_quote_tuples)| {
                let part_quote_items = part_quote_tuples
                    .into_iter()
                    .map(|part_quote| {
                        let item = to_item(part_quote).expect("error converting to dynamodb item");
                        AttributeValue::M(item)
                    })
                    .collect();

                let selected_part_quote_id = selected_part_quote_by_part
                    .get(&part_id)
                    .expect("expecting a selected part quote")
                    .clone();

                TransactWriteItem::builder()
                    .update(
                        Update::builder()
                            .key("quotation_id", AttributeValue::S(quotation_id.clone()))
                            .key("id", AttributeValue::S(part_id))
                            .table_name(&self.parts_table)
                            .set_expression_attribute_values(Some(
                                [
                                    (
                                        String::from(":part_quotes"),
                                        AttributeValue::L(part_quote_items),
                                    ),
                                    (
                                        String::from(":selected_part_quote_id"),
                                        AttributeValue::S(selected_part_quote_id),
                                    ),
                                ]
                                    .into_iter()
                                    .collect::<HashMap<String, AttributeValue>>(),
                            ))
                            .update_expression("SET part_quotes = :part_quotes, selected_part_quote_id = :selected_part_quote_id")
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect()
    }
}
