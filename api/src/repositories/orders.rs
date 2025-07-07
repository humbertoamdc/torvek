use crate::orders::models::order::Order;
use crate::shared::money::Money;
use crate::shared::{QueryResponse, Result};
use async_trait::async_trait;

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn query_orders_for_customer(
        &self,
        customer_id: String,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Order>, String>>;
    async fn query_open_orders(&self) -> Result<QueryResponse<Vec<Order>, String>>;
    async fn update_order_payout(&self, order_id: String, payout: Money) -> Result<()>;
}
