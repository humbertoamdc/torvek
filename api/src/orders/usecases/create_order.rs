use std::sync::Arc;

use axum::async_trait;

use api_boundary::orders::models::{Order, OrderStatus};
use api_boundary::orders::requests::AdminCreateOrdersRequest;

use crate::orders::domain::errors::OrdersError;
use crate::orders::services::orders_creation::OrdersCreationService;
use crate::shared::usecase::UseCase;

pub struct AdminCreateOrderUseCase {
    orders_creation_service: Arc<dyn OrdersCreationService>,
}

impl AdminCreateOrderUseCase {
    pub fn new(orders_creation_service: Arc<dyn OrdersCreationService>) -> Self {
        Self {
            orders_creation_service,
        }
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

        self.orders_creation_service
            .create_orders_and_update_quotation_status(
                request.client_id,
                request.project_id,
                request.quotation_id,
                orders,
            )
            .await
    }
}
