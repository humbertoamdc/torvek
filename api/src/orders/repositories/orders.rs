use crate::orders::domain::errors::OrdersError;
use api_boundary::orders::models::Order;
use axum::async_trait;

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn create_orders(&self, orders: Vec<Order>) -> Result<(), OrdersError>;
}
