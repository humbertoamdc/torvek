use std::sync::Arc;

use api_boundary::orders::errors::OrdersError;
use api_boundary::orders::requests::QueryOpenOrdersRequest;
use axum::async_trait;

use api_boundary::orders::responses::QueryOpenOrdersResponse;

use crate::orders::repositories::orders::OrdersRepository;
use crate::shared::usecase::UseCase;

pub struct QueryOpenOrdersUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl QueryOpenOrdersUseCase {
    pub fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<QueryOpenOrdersRequest, QueryOpenOrdersResponse, OrdersError>
    for QueryOpenOrdersUseCase
{
    async fn execute(
        &self,
        _: QueryOpenOrdersRequest,
    ) -> Result<QueryOpenOrdersResponse, OrdersError> {
        let orders = self.orders_repository.query_open_orders().await?;

        Ok(QueryOpenOrdersResponse { orders })
    }
}
