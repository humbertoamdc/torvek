use crate::orders::models::order::Order;
use crate::shared::money::Money;
use crate::shared::{QueryResponse, Result};
use async_trait::async_trait;

pub enum QueryBy {
    Customer,
    IsOpen,
}

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn query(
        &self,
        customer_id: Option<String>,
        query_by: QueryBy,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Order>, String>>;
    async fn update(&self, order_id: String, payout: Option<Money>) -> Result<()>;
}
