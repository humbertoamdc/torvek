use axum::async_trait;

use api_boundary::orders::models::Order;

use crate::payments::domain::errors::PaymentsError;

#[async_trait]
pub trait OrdersCreationService: Send + Sync + 'static {
    async fn create_orders_and_update_quotation_status(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
        orders: Vec<Order>,
    ) -> Result<(), PaymentsError>;
}
