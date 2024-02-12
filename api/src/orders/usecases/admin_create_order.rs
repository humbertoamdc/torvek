use axum::async_trait;

use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::orders::requests::AdminCreateOrdersRequest;

use crate::orders::domain::errors::OrdersError;
use crate::shared::usecase::UseCase;

pub struct AdminCreateOrderUseCase {}

impl AdminCreateOrderUseCase {
    pub fn _new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UseCase<AdminCreateOrdersRequest, (), OrdersError> for AdminCreateOrderUseCase {
    async fn execute(&self, request: AdminCreateOrdersRequest) -> Result<(), OrdersError> {
        let _orders: Vec<Order> = request
            .data
            .into_iter()
            .map(|order| {
                Order::new(
                    order.part_id,
                    order.model_file,
                    Some(order.payment),
                    order.deadline,
                    OrderStatus::Open,
                )
            })
            .collect();

        // TODO: Change logic and naming to update order with payment.

        Ok(())
    }
}
