use crate::orders::models::dynamodb_order_item::DynamodbOrderItem;
use crate::orders::models::order::Order;
use crate::payments::services::orders_creation::OrdersCreationService;
use crate::quotations::models::quotation::QuoteStatus;
use crate::shared::error::Error;
use crate::shared::Result;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::{AttributeValue, Put, TransactWriteItem, Update};
use serde_dynamo::to_item;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DynamodbOrdersCreationService {
    client: aws_sdk_dynamodb::Client,
    orders_table: String,
    projects_table: String,
    quotations_table: String,
}

impl DynamodbOrdersCreationService {
    pub fn new(
        client: aws_sdk_dynamodb::Client,
        orders_table: String,
        projects_table: String,
        quotations_table: String,
    ) -> Self {
        Self {
            client,
            orders_table,
            projects_table,
            quotations_table,
        }
    }
}

#[async_trait]
impl OrdersCreationService for DynamodbOrdersCreationService {
    async fn create_orders_and_update_quotation_status(
        &self,
        customer_id: String,
        project_id: String,
        quotation_id: String,
        orders: Vec<Order>,
    ) -> Result<()> {
        // Parse to DynamoDB format.
        let items = orders
            .into_iter()
            .map(|order| DynamodbOrderItem::from(order))
            .collect::<Vec<_>>();

        // Update project status to Locked.
        let project_transaction = self.build_project_transaction(customer_id, project_id.clone());

        // Update quotation status to OrdersCreated.
        let quotation_transaction = self.build_quotation_transaction(project_id, quotation_id);

        // Create orders Dynamodb items.
        let orders_transactions = self.build_orders_transactions(items);

        // Build transaction request.
        let mut transaction_request = self
            .client
            .transact_write_items()
            .transact_items(project_transaction)
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
    fn build_project_transaction(
        &self,
        customer_id: String,
        project_id: String,
    ) -> TransactWriteItem {
        TransactWriteItem::builder()
            .update(
                Update::builder()
                    .table_name(&self.projects_table)
                    .set_key(Some(HashMap::from([
                        (String::from("customer_id"), AttributeValue::S(customer_id)),
                        (String::from("id"), AttributeValue::S(project_id)),
                    ])))
                    .update_expression("SET is_locked = :is_locked, updated_at = :updated_at")
                    .set_expression_attribute_values(Some(HashMap::from([
                        (String::from(":is_locked"), AttributeValue::Bool(true)),
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
                            AttributeValue::S(QuoteStatus::Payed.to_string()),
                        ),
                        (
                            String::from(":awaitingPaymentStatus"),
                            AttributeValue::S(QuoteStatus::PendingPayment.to_string()),
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
