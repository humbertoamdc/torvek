use axum::async_trait;

use api_boundary::common::money::Money;
use api_boundary::orders::errors::OrdersError;
use api_boundary::orders::models::{Order, OrderStatus};

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn query_orders_by_status(&self, status: OrderStatus) -> Result<Vec<Order>, OrdersError>;
    async fn update_order_payout(&self, order_id: String, payout: Money)
        -> Result<(), OrdersError>;
}
