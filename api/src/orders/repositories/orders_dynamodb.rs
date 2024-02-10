use crate::orders::domain::errors::OrdersError;
use crate::orders::repositories::orders::OrdersRepository;
use api_boundary::orders::models::Order;
use aws_sdk_dynamodb::types::{PutRequest, WriteRequest};
use axum::async_trait;
use serde_dynamo::to_item;

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
                log::error!("{err:?}");
                Err(OrdersError::CreateOrdersError)
            }
        }
    }
}
