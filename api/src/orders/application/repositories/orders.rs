use axum::async_trait;

use crate::orders::domain::errors::OrdersError;
use crate::orders::domain::order::{Order, OrderStatus, UpdatableOrder};

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn query_orders_for_client(&self, client_id: String) -> Result<Vec<Order>, OrdersError>;
    async fn query_orders_by_status(
        &self,
        order_status: OrderStatus,
    ) -> Result<Vec<Order>, OrdersError>;
    async fn create_orders(&self, orders: Vec<Order>) -> Result<(), OrdersError>;
    async fn update_order(
        &self,
        client_id: String,
        order_id: String,
        order: UpdatableOrder,
    ) -> Result<(), OrdersError>;
}
