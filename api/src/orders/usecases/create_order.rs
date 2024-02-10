use crate::orders::domain::errors::OrdersError;
use crate::orders::repositories::orders::OrdersRepository;
use crate::shared::usecase::UseCase;
use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::orders::requests::AdminCreateOrdersRequest;
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
impl UseCase<AdminCreateOrdersRequest, (), OrdersError> for AdminCreateOrderUseCase {
    async fn execute(&self, request: AdminCreateOrdersRequest) -> Result<(), OrdersError> {
        let orders = request
            .data
            .into_iter()
            .map(|order| {
                Order::new(
                    order.part_id,
                    order.model_file,
                    order.payment,
                    order.deadline,
                    OrderStatus::Open,
                )
            })
            .collect();

        self.orders_repository.create_orders(orders).await
    }
}
