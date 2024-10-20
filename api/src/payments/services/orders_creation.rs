use axum::async_trait;

use crate::shared::Result;
use api_boundary::orders::models::Order;

#[async_trait]
pub trait OrdersCreationService: Send + Sync + 'static {
    async fn create_orders_and_update_quotation_status(
        &self,
        project_id: String,
        quotation_id: String,
        orders: Vec<Order>,
    ) -> Result<()>;
}
