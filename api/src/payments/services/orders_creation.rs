use crate::orders::models::order::Order;
use crate::shared::{CustomerId, ProjectId, QuoteId, Result};
use async_trait::async_trait;

#[async_trait]
pub trait OrdersCreationService: Send + Sync + 'static {
    async fn create_orders_and_update_quotation_status(
        &self,
        customer_id: CustomerId,
        project_id: ProjectId,
        quotation_id: QuoteId,
        orders: Vec<Order>,
    ) -> Result<()>;
}
