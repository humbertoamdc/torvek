use std::sync::Arc;

use api_boundary::orders::errors::OrdersError;
use axum::async_trait;

use api_boundary::orders::requests::QueryOrdersByStatusRequest;
use api_boundary::orders::responses::QueryOrdersByStatusResponse;

use crate::orders::repositories::orders::OrdersRepository;
use crate::shared::usecase::UseCase;

pub struct QueryOrdersByStatusUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl QueryOrdersByStatusUseCase {
    pub fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<QueryOrdersByStatusRequest, QueryOrdersByStatusResponse, OrdersError>
    for QueryOrdersByStatusUseCase
{
    async fn execute(
        &self,
        request: QueryOrdersByStatusRequest,
    ) -> Result<QueryOrdersByStatusResponse, OrdersError> {
        let orders = self
            .orders_repository
            .query_orders_by_status(request.status)
            .await?;

        Ok(QueryOrdersByStatusResponse { orders })
    }
}
