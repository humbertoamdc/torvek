use std::sync::Arc;

use axum::async_trait;

use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::orders::requests::StripeCreateOrdersRequest;

use crate::orders::domain::errors::OrdersError;
use crate::orders::repositories::orders::OrdersRepository;
use crate::shared::usecase::UseCase;

pub struct CreateOrdersUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl CreateOrdersUseCase {
    pub fn _new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<StripeCreateOrdersRequest, (), OrdersError> for CreateOrdersUseCase {
    async fn execute(&self, request: StripeCreateOrdersRequest) -> Result<(), OrdersError> {
        let orders = request
            .data
            .into_iter()
            .map(|order| {
                Order::new(
                    order.part_id,
                    order.model_file,
                    None,
                    order.deadline,
                    OrderStatus::Open,
                )
            })
            .collect();

        self.orders_repository.create_orders(orders).await
    }
}
