use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use axum::async_trait;
use chrono::Utc;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;
use serde_dynamo::from_items;

use crate::orders::domain::dynamodb_order_item::DynamodbOrderItem;
use api_boundary::common::money::Money;
use api_boundary::orders::errors::OrdersError;
use api_boundary::orders::models::{Order, OrderStatus};

use crate::orders::repositories::orders::OrdersRepository;

static OPEN_ORDERS_INDEX: &'static str = "OpenOrders";

#[derive(Clone)]
pub struct DynamodbOrders {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbOrders {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl OrdersRepository for DynamodbOrders {
    async fn query_orders_for_customer(
        &self,
        customer_id: String,
    ) -> Result<Vec<Order>, OrdersError> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .key_condition_expression("customer_id = :value")
            .expression_attribute_values(":value", AttributeValue::S(customer_id))
            .scan_index_forward(false)
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_orders) => {
                        let orders = dynamodb_orders
                            .into_iter()
                            .map(|dynamodb_order: DynamodbOrderItem| dynamodb_order.into())
                            .collect();
                        Ok(orders)
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                        Err(OrdersError::UnknownError)
                    }
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
                Err(OrdersError::UnknownError)
            }
        }
    }

    async fn query_open_orders(&self) -> Result<Vec<Order>, OrdersError> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .index_name(OPEN_ORDERS_INDEX)
            .key_condition_expression("is_open = :value")
            .expression_attribute_values(":value", AttributeValue::S(String::from("1")))
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_orders) => {
                        let orders = dynamodb_orders
                            .into_iter()
                            .map(|dynamodb_order: DynamodbOrderItem| dynamodb_order.into())
                            .collect();
                        Ok(orders)
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                        Err(OrdersError::UnknownError)
                    }
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
                Err(OrdersError::UnknownError)
            }
        }
    }

    async fn update_order_payout(
        &self,
        order_id: String,
        payout: Money,
    ) -> Result<(), OrdersError> {
        let response = self
            .client
            .update_item()
            .table_name(&self.table)
            .key("id", AttributeValue::S(order_id))
            .condition_expression("#status = :pendingPricingStatus")
            .update_expression(
                "SET payout = :payout, #status = :openStatus, updated_at = :updated_at",
            )
            .set_expression_attribute_values(Some(HashMap::from([
                (
                    String::from(":payout"),
                    AttributeValue::M(to_item(&payout).unwrap()),
                ),
                (
                    String::from(":pendingPricingStatus"),
                    AttributeValue::S(OrderStatus::PendingPricing.to_string()),
                ),
                (
                    String::from(":openStatus"),
                    AttributeValue::S(OrderStatus::Open.to_string()),
                ),
                (
                    String::from(":updated_at"),
                    AttributeValue::S(Utc::now().to_rfc3339()),
                ),
            ])))
            .expression_attribute_names("#status", "status")
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{:?}", err);
                Err(OrdersError::UnknownError)
            }
        }
    }
}
