use crate::orders::models::responses::QueryOpenOrdersResponse;
use crate::repositories::orders::OrdersRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct QueryOpenOrdersUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl QueryOpenOrdersUseCase {
    pub fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<(), QueryOpenOrdersResponse> for QueryOpenOrdersUseCase {
    async fn execute(&self, _: ()) -> Result<QueryOpenOrdersResponse> {
        let response = self.orders_repository.query_open_orders().await?;

        Ok(QueryOpenOrdersResponse {
            orders: response.data,
            cursor: response.cursor,
        })
    }
}
