use axum::async_trait;

use api_boundary::orders::models::{Order, OrderStatus};

use crate::orders::domain::errors::OrdersError;

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn query_orders_by_status(&self, status: OrderStatus) -> Result<Vec<Order>, OrdersError>;
}
