use std::collections::HashMap;

use api_boundary::common::error::Error;
use aws_sdk_dynamodb::types::{AttributeValue, Put, TransactWriteItem, Update};
use axum::async_trait;
use serde_dynamo::to_item;

use crate::orders::domain::dynamodb_order_item::DynamodbOrderItem;
use crate::shared::Result;
use api_boundary::orders::models::Order;
use api_boundary::quotations::models::QuotationStatus;

use crate::payments::services::orders_creation::OrdersCreationService;

#[derive(Clone)]
pub struct DynamodbOrdersCreationService {
    client: aws_sdk_dynamodb::Client,
    orders_table: String,
    quotations_table: String,
}

impl DynamodbOrdersCreationService {
    pub fn new(
        client: aws_sdk_dynamodb::Client,
        orders_table: String,
        quotations_table: String,
    ) -> Self {
        Self {
            client,
            orders_table,
            quotations_table,
        }
    }
}

#[async_trait]
impl OrdersCreationService for DynamodbOrdersCreationService {
    async fn create_orders_and_update_quotation_status(
        &self,
        project_id: String,
        quotation_id: String,
        orders: Vec<Order>,
    ) -> Result<()> {
        // Parse to DynamoDB format.
        let items = orders
            .into_iter()
            .map(|order| DynamodbOrderItem::from(order))
            .collect::<Vec<_>>();

        // Update quotation status to OrdersCreated.
        let quotation_transaction = self.build_quotation_transaction(project_id, quotation_id);

        // Create orders Dynamodb items.
        let orders_transactions = self.build_orders_transactions(items);

        // Build transaction request.
        let mut transaction_request = self
            .client
            .transact_write_items()
            .transact_items(quotation_transaction);

        for transaction in orders_transactions {
            transaction_request = transaction_request.transact_items(transaction.clone());
        }

        let response = transaction_request.send().await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}

impl DynamodbOrdersCreationService {
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
                    .condition_expression("#status = :awaitingPaymentStatus")
                    .update_expression("SET #status = :payedStatus, updated_at = :updated_at")
                    .set_expression_attribute_names(Some(HashMap::from([(
                        String::from("#status"),
                        String::from("status"),
                    )])))
                    .set_expression_attribute_values(Some(HashMap::from([
                        (
                            String::from(":payedStatus"),
                            AttributeValue::S(QuotationStatus::Payed.to_string()),
                        ),
                        (
                            String::from(":awaitingPaymentStatus"),
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

    fn build_orders_transactions(&self, orders: Vec<DynamodbOrderItem>) -> Vec<TransactWriteItem> {
        orders
            .into_iter()
            .map(|order| {
                TransactWriteItem::builder()
                    .put(
                        Put::builder()
                            .set_item(Some(
                                to_item(order).expect("error converting to dynamodb item"),
                            ))
                            .table_name(&self.orders_table)
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect::<Vec<TransactWriteItem>>()
    }
}
