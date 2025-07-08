use crate::orders::models::responses::QueryOpenOrdersResponse;
use crate::repositories::orders::{OrdersRepository, QueryBy};
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct QueryOpenOrders {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl QueryOpenOrders {
    pub fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<(), QueryOpenOrdersResponse> for QueryOpenOrders {
    async fn execute(&self, _: ()) -> Result<QueryOpenOrdersResponse> {
        let response = self
            .orders_repository
            .query(None, QueryBy::IsOpen, None, 100)
            .await?;

        Ok(QueryOpenOrdersResponse {
            orders: response.data,
            cursor: response.cursor,
        })
    }
}
