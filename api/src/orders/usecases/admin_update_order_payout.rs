use crate::repositories::orders::OrdersRepository;
use api_boundary::orders::requests::AdminUpdateOrderPayoutRequest;
use async_trait::async_trait;
use std::sync::Arc;

use crate::shared::{Result, UseCase};

pub struct AdminUpdateOrderPayoutUsecase {
    pub orders_repository: Arc<dyn OrdersRepository>,
}

impl AdminUpdateOrderPayoutUsecase {
    pub fn new(orders_repository: Arc<dyn OrdersRepository>) -> Self {
        Self { orders_repository }
    }
}

#[async_trait]
impl UseCase<AdminUpdateOrderPayoutRequest, ()> for AdminUpdateOrderPayoutUsecase {
    async fn execute(&self, request: AdminUpdateOrderPayoutRequest) -> Result<()> {
        self.orders_repository
            .update_order_payout(request.order_id, request.payout)
            .await
    }
}
