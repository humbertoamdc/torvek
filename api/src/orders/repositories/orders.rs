use api_boundary::common::error::Error;
use axum::async_trait;

use api_boundary::common::money::Money;
use api_boundary::orders::models::Order;

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {
    async fn query_orders_for_customer(&self, customer_id: String) -> Result<Vec<Order>, Error>;
    async fn query_open_orders(&self) -> Result<Vec<Order>, Error>;
    async fn update_order_payout(&self, order_id: String, payout: Money) -> Result<(), Error>;
}
