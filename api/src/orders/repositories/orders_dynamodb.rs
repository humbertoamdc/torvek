use crate::orders::domain::errors::OrdersError;
use crate::orders::repositories::orders::OrdersRepository;
use api_boundary::orders::models::Order;
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
    async fn create_order(&self, order: Order) -> Result<(), OrdersError> {
        let item = to_item(order).expect("error converting to dynamodb item");
        let response = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{err:?}");
                Err(OrdersError::CreateOrderError)
            }
        }
    }
}
