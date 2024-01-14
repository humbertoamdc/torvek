use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
use axum::async_trait;
use chrono::Utc;
use serde_dynamo::{from_items, to_item};

use crate::orders::application::repositories::orders::OrdersRepository;
use crate::orders::domain::errors::OrdersError;
use crate::orders::domain::order::{Order, OrderStatus, UpdatableOrder};

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
    async fn query_orders_for_client(&self, client_id: String) -> Result<Vec<Order>, OrdersError> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .key_condition_expression("client_id = :client_id")
            .expression_attribute_values(":client_id", AttributeValue::S(client_id))
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(orders) => Ok(orders),
                    Err(_) => Err(OrdersError::ConversionError),
                }
            }
            Err(_) => Err(OrdersError::QueryOrdersError),
        }
    }

    async fn query_orders_by_status(
        &self,
        order_status: OrderStatus,
    ) -> Result<Vec<Order>, OrdersError> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .index_name("OrdersByStatus")
            .key_condition_expression("order_status = :order_status")
            .expression_attribute_values(
                ":order_status",
                AttributeValue::S(order_status.to_string()),
            )
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(orders) => Ok(orders),
                    Err(err) => {
                        log::error!("{err:?}");
                        Err(OrdersError::ConversionError)
                    }
                }
            }
            Err(err) => {
                log::error!("{err:#?}");
                Err(OrdersError::QueryOrdersError)
            }
        }
    }

    async fn create_orders(&self, orders: Vec<Order>) -> Result<(), OrdersError> {
        // TODO: Extract dynamodb item mapping to a mapper.
        let items: Vec<WriteRequest> = orders
            .into_iter()
            .map(|order| {
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(
                                to_item(order).expect("error converting to dynamodb item"),
                            ))
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect();

        let response = self
            .client
            .batch_write_item()
            .request_items(&self.table, items)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(OrdersError::OrdersBatchCreateError),
        }
    }

    async fn update_order(
        &self,
        client_id: String,
        order_id: String,
        order: UpdatableOrder,
    ) -> Result<(), OrdersError> {
        let mut update_expression = String::from("SET ");
        let mut expression_attribute_values = std::collections::HashMap::new();

        update_expression.push_str("updated_at = :updated_at, ");
        expression_attribute_values.insert(
            ":updated_at".to_string(),
            AttributeValue::S(Utc::now().to_string()),
        );

        if let Some(drawing_file_name) = order.drawing_file_name {
            update_expression.push_str("drawing_file_name = :drawing_file_name, ");
            expression_attribute_values.insert(
                ":drawing_file_name".to_string(),
                AttributeValue::S(drawing_file_name),
            );
        }
        if let Some(drawing_file_url) = order.drawing_file_url {
            update_expression.push_str("drawing_file_url = :drawing_file_url, ");
            expression_attribute_values.insert(
                ":drawing_file_url".to_string(),
                AttributeValue::S(drawing_file_url),
            );
        }
        if let Some(process) = order.process {
            update_expression.push_str("process = :process, ");
            expression_attribute_values.insert(":process".to_string(), AttributeValue::S(process));
        }
        if let Some(material) = order.material {
            update_expression.push_str("material = :material, ");
            expression_attribute_values
                .insert(":material".to_string(), AttributeValue::S(material));
        }
        if let Some(tolerance) = order.tolerance {
            update_expression.push_str("tolerance = :tolerance, ");
            expression_attribute_values
                .insert(":tolerance".to_string(), AttributeValue::S(tolerance));
        }
        if let Some(quantity) = order.quantity {
            update_expression.push_str("quantity = :quantity, ");
            expression_attribute_values.insert(
                ":quantity".to_string(),
                AttributeValue::N(quantity.to_string()),
            );
        }
        if let Some(unit_price) = order.unit_price {
            update_expression.push_str("unit_price = :unit_price, ");
            expression_attribute_values.insert(
                ":unit_price".to_string(),
                AttributeValue::N(unit_price.to_string()),
            );
        }
        if let Some(sub_total) = order.sub_total {
            update_expression.push_str("sub_total = :sub_total, ");
            expression_attribute_values.insert(
                ":sub_total".to_string(),
                AttributeValue::N(sub_total.to_string()),
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
            .key("client_id", AttributeValue::S(client_id))
            .key("id", AttributeValue::S(order_id))
            .update_expression(update_expression)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(OrdersError::UpdateOrderError),
        }
    }
}
