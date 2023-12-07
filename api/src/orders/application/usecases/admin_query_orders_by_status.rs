use std::sync::Arc;

use axum::async_trait;

use crate::orders::adapters::api::requests::AdminQueryOrdersByStatusRequest;
use crate::orders::adapters::api::responses::AdminQueryOrderByStatusResponse;
use crate::orders::application::repositories::orders::OrdersRepository;
use crate::orders::application::usecases::interfaces::UseCase;
use crate::orders::domain::errors::OrdersError;

pub struct AdminQueryOrdersByStatusUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl AdminQueryOrdersByStatusUseCase {
    pub const fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<AdminQueryOrdersByStatusRequest, AdminQueryOrderByStatusResponse, OrdersError>
    for AdminQueryOrdersByStatusUseCase
{
    async fn execute(
        &self,
        request: AdminQueryOrdersByStatusRequest,
    ) -> Result<AdminQueryOrderByStatusResponse, OrdersError> {
        let orders = self
            .orders_repository
            .query_orders_by_status(request.order_status)
            .await?;
        Ok(AdminQueryOrderByStatusResponse::new(orders))
    }
}
