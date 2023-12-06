use std::sync::Arc;

use axum::async_trait;

use crate::orders::adapters::api::requests::QueryOrdersForClientRequest;
use crate::orders::adapters::api::responses::QueryOrdersForClientResponse;
use crate::orders::application::repositories::orders::OrdersRepository;
use crate::orders::application::usecases::interfaces::UseCase;
use crate::orders::domain::errors::OrdersError;

pub struct QueryOrdersForClientUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl QueryOrdersForClientUseCase {
    pub const fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<QueryOrdersForClientRequest, QueryOrdersForClientResponse, OrdersError>
    for QueryOrdersForClientUseCase
{
    async fn execute(
        &self,
        request: QueryOrdersForClientRequest,
    ) -> Result<QueryOrdersForClientResponse, OrdersError> {
        let orders = self
            .orders_repository
            .query_orders_for_client(request.client_id)
            .await?;
        Ok(QueryOrdersForClientResponse::new(orders))
    }
}
