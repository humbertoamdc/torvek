use crate::orders::domain::errors::OrdersError;
use crate::orders::repositories::orders::OrdersRepository;
use crate::shared::usecase::UseCase;
use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::orders::requests::AdminCreateOrderRequest;
use axum::async_trait;
use std::sync::Arc;

pub struct AdminCreateOrderUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl AdminCreateOrderUseCase {
    pub fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<AdminCreateOrderRequest, (), OrdersError> for AdminCreateOrderUseCase {
    async fn execute(&self, request: AdminCreateOrderRequest) -> Result<(), OrdersError> {
        let order = Order::new(
            request.part_id,
            request.model_file,
            request.payment,
            request.deadline,
            OrderStatus::Open,
        );
        self.orders_repository.create_order(order).await
    }
}
