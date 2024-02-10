use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, Put, TransactWriteItem, Update};
use axum::async_trait;
use serde_dynamo::to_item;

use api_boundary::orders::models::Order;
use api_boundary::quotations::models::QuotationStatus;

use crate::orders::domain::errors::OrdersError;
use crate::orders::services::orders_creation::OrdersCreationService;

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
        client_id: String,
        project_id: String,
        quotation_id: String,
        orders: Vec<Order>,
    ) -> Result<(), OrdersError> {
        // Update quotation status to OrdersCreated.
        let client_id_and_project_id = format!("{client_id}#{project_id}");
        let quotation_transaction = TransactWriteItem::builder().update(
            Update::builder()
                .table_name(&self.quotations_table)
                .set_key(Some(HashMap::from([
                    (
                        String::from("client_id#project_id"),
                        AttributeValue::S(client_id_and_project_id),
                    ),
                    (String::from("id"), AttributeValue::S(quotation_id)),
                ])))
                .condition_expression("#status = :payedStatus")
                .update_expression("SET #status = :ordersCreatedStatus")
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
                        String::from(":ordersCreatedStatus"),
                        AttributeValue::S(QuotationStatus::OrdersCreated.to_string()),
                    ),
                ])))
                .build()
                .unwrap(),
        );

        // Create orders Dynamodb items.
        let mut orders_transaction = TransactWriteItem::builder();
        for order in orders {
            let put = Put::builder()
                .set_item(Some(
                    to_item(order).expect("error converting to dynamodb item"),
                ))
                .table_name(&self.orders_table)
                .build()
                .unwrap();
            orders_transaction = orders_transaction.put(put);
        }

        let response = self
            .client
            .transact_write_items()
            .transact_items(quotation_transaction.build())
            .transact_items(orders_transaction.build())
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{err:?}");
                Err(OrdersError::CreateOrdersError)
            }
        }
    }
}
