use api_boundary::orders::requests::QueryOpenOrdersRequest;
use api_boundary::orders::responses::QueryOpenOrdersResponse;
use async_trait::async_trait;
use std::sync::Arc;

use crate::repositories::orders::OrdersRepository;
use crate::shared::{Result, UseCase};

pub struct QueryOpenOrdersUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl QueryOpenOrdersUseCase {
    pub fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<QueryOpenOrdersRequest, QueryOpenOrdersResponse> for QueryOpenOrdersUseCase {
    async fn execute(&self, _: QueryOpenOrdersRequest) -> Result<QueryOpenOrdersResponse> {
        let orders = self.orders_repository.query_open_orders().await?;

        Ok(QueryOpenOrdersResponse { orders })
    }
}
