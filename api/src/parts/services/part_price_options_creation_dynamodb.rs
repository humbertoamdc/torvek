use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, Put, TransactWriteItem, Update};
use axum::async_trait;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;

use api_boundary::parts::models::PartPriceOption;
use api_boundary::quotations::models::QuotationStatus;

use crate::parts::domain::errors::PartsError;
use crate::parts::services::part_price_options_creation::PartPriceOptionsCreation;

pub struct DynamodbPartPriceOptionsCreation {
    client: aws_sdk_dynamodb::Client,
    part_price_options_table: String,
    quotations_table: String,
}

impl DynamodbPartPriceOptionsCreation {
    pub fn new(
        client: aws_sdk_dynamodb::Client,
        part_price_options_table: String,
        quotations_table: String,
    ) -> Self {
        Self {
            client,
            part_price_options_table,
            quotations_table,
        }
    }
}

#[async_trait]
impl PartPriceOptionsCreation for DynamodbPartPriceOptionsCreation {
    async fn create_part_price_options(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
        part_price_options: Vec<PartPriceOption>,
    ) -> Result<(), PartsError> {
        // Update quotation status to PendingPayment..
        let quotation_transaction =
            self.build_quotation_transaction(client_id, project_id, quotation_id);

        // Create part price options Dynamodb items.
        let part_price_options_transactions =
            self.build_part_price_options_transaction(part_price_options);

        // Build transaction request.
        let mut transaction_request = self
            .client
            .transact_write_items()
            .transact_items(quotation_transaction);

        for transaction in part_price_options_transactions {
            transaction_request = transaction_request.transact_items(transaction.clone());
        }

        let response = transaction_request.send().await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => {
                log::error!("{:?}", error);
                Err(PartsError::CreatePartsPriceOptionsAndUpdateQuotationStatusTransactionError)
            }
        }
    }
}

impl DynamodbPartPriceOptionsCreation {
    fn build_quotation_transaction(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> TransactWriteItem {
        let client_id_and_project_id = format!("{client_id}#{project_id}");

        TransactWriteItem::builder()
            .update(
                Update::builder()
                    .table_name(&self.quotations_table)
                    .set_key(Some(HashMap::from([
                        (
                            String::from("client_id#project_id"),
                            AttributeValue::S(client_id_and_project_id),
                        ),
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

    fn build_part_price_options_transaction(
        &self,
        part_price_options: Vec<PartPriceOption>,
    ) -> Vec<TransactWriteItem> {
        part_price_options
            .into_iter()
            .map(|part_price_option| {
                TransactWriteItem::builder()
                    .put(
                        Put::builder()
                            .set_item(Some(
                                to_item(part_price_option)
                                    .expect("error converting to dynamodb item"),
                            ))
                            .table_name(&self.part_price_options_table)
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect()
    }
}
