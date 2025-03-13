use crate::shared::Result;
use api_boundary::orders::models::Order;
use async_trait::async_trait;

#[async_trait]
pub trait OrdersCreationService: Send + Sync + 'static {
    async fn create_orders_and_update_quotation_status(
        &self,
        customer_id: String,
        project_id: String,
        quotation_id: String,
        orders: Vec<Order>,
    ) -> Result<()>;
}
