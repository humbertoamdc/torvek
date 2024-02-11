use aws_sdk_dynamodb::types::AttributeValue;
use axum::async_trait;
use serde_dynamo::from_items;

use api_boundary::orders::models::{Order, OrderStatus};

use crate::orders::domain::errors::OrdersError;
use crate::orders::repositories::orders::OrdersRepository;

static ORDERS_BY_STATUS_INDEX: &'static str = "OrdersByStatus";

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
    async fn query_orders_by_status(&self, status: OrderStatus) -> Result<Vec<Order>, OrdersError> {
        let response = self
            .client
            .query()
            .table_name(&self.table)
            .index_name(ORDERS_BY_STATUS_INDEX)
            .key_condition_expression("#status = :value")
            .expression_attribute_values(":value", AttributeValue::S(status.to_string()))
            .expression_attribute_names("#status", "status")
            .send()
            .await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items(items) {
                    Ok(orders) => Ok(orders),
                    Err(err) => {
                        log::error!("{:?}", err);
                        Err(OrdersError::UnknownError)
                    }
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
                Err(OrdersError::QueryOrdersError)
            }
        }
    }
}
