use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
use axum::async_trait;
use serde_dynamo::{from_items, to_item};

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
    async fn create_orders(&self, orders: Vec<Order>) -> Result<(), OrdersError> {
        let items = orders
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
            Err(err) => {
                log::error!("{:?}", err);
                Err(OrdersError::CreateOrdersBatchError)
            }
        }
    }

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
