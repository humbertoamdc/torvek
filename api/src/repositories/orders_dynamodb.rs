use crate::orders::models::dynamodb_order_item::DynamodbOrderItem;
use crate::orders::models::order::{Order, OrderStatus};
use crate::repositories::orders::{OrdersRepository, QueryBy};
use crate::shared::error::Error;
use crate::shared::money::Money;
use crate::shared::{QueryResponse, Result};
use crate::utils::dynamodb_key_codec::DynamodbKeyCodec;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;
use serde_dynamo::from_items;
use std::collections::HashMap;

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
    async fn query(
        &self,
        customer_id: Option<String>,
        query_by: QueryBy,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Order>, String>> {
        let mut query = self
            .client
            .query()
            .table_name(&self.table)
            .limit(limit)
            .set_exclusive_start_key(DynamodbKeyCodec::decode_from_base64(cursor));

        match query_by {
            QueryBy::Customer => {
                let customer_id = customer_id
                    .ok_or(Error::MissingRequiredParameter(String::from("customer_id")))?;

                query = query
                    .key_condition_expression("customer_id = :value")
                    .expression_attribute_values(":value", AttributeValue::S(customer_id))
                    .scan_index_forward(false);
            }
            QueryBy::IsOpen => {
                query = query
                    .index_name(OPEN_ORDERS_INDEX)
                    .key_condition_expression("is_open = :value")
                    .expression_attribute_values(":value", AttributeValue::S(String::from("1")))
            }
        };

        let response = query.send().await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(dynamodb_orders) => {
                        let orders = dynamodb_orders
                            .into_iter()
                            .map(|dynamodb_order: DynamodbOrderItem| dynamodb_order.into())
                            .collect();
                        Ok(QueryResponse {
                            data: orders,
                            cursor: DynamodbKeyCodec::encode_to_base64(output.last_evaluated_key()),
                        })
                    }
                    Err(err) => {
                        tracing::error!("{:?}", err);
                        Err(Error::UnknownError)
                    }
                }
            }
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(Error::UnknownError)
            }
        }
    }

    async fn update(&self, order_id: String, payout: Option<Money>) -> Result<()> {
        let mut update_expression =
            String::from("SET updated_at = :updated_at, #status = :openStatus, ");
        let mut expression_attribute_values: HashMap<String, AttributeValue> = [
            (
                String::from(":updated_at"),
                AttributeValue::S(Utc::now().to_rfc3339()),
            ),
            (
                String::from(":openStatus"),
                AttributeValue::S(OrderStatus::Open.to_string()),
            ),
            (
                String::from(":pendingPricingStatus"),
                AttributeValue::S(OrderStatus::PendingPricing.to_string()),
            ),
        ]
        .into_iter()
        .collect();
        let expression_attribute_names = [(String::from("#status"), String::from("status"))]
            .into_iter()
            .collect();

        if let Some(payout) = payout {
            update_expression.push_str("payout = :payout, ");
            expression_attribute_values.insert(
                String::from(":payout"),
                AttributeValue::M(to_item(&payout).unwrap()),
            );
        }

        // Remove trailing comma and space
        if !update_expression.is_empty() {
            update_expression.pop();
            update_expression.pop();
        }

        let response = self
            .client
            .update_item()
            .table_name(&self.table)
            .key("id", AttributeValue::S(order_id))
            .condition_expression("#status = :pendingPricingStatus")
            .update_expression(update_expression)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .set_expression_attribute_names(Some(expression_attribute_names))
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{:?}", err);
                Err(Error::UnknownError)
            }
        }
    }
}
