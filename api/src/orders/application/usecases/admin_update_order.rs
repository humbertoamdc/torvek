use std::sync::Arc;

use axum::async_trait;

use crate::orders::adapters::api::requests::AdminUpdateOrderRequest;
use crate::orders::application::repositories::orders::OrdersRepository;
use crate::orders::application::usecases::interfaces::UseCase;
use crate::orders::domain::errors::OrdersError;
use crate::orders::domain::order::UpdatableOrder;

pub struct AdminUpdateOrderUseCase {
    orders_repository: Arc<dyn OrdersRepository>,
}

impl AdminUpdateOrderUseCase {
    pub const fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<AdminUpdateOrderRequest, (), OrdersError> for AdminUpdateOrderUseCase {
    async fn execute(&self, request: AdminUpdateOrderRequest) -> Result<(), OrdersError> {
        let updatable_order = UpdatableOrder::from(&request);
        self.orders_repository
            .update_order(
                request.client_id.clone(),
                request.order_id.clone(),
                updatable_order,
            )
            .await
    }
}
