use axum::async_trait;

use api_boundary::parts::models::PartPriceOption;

use crate::parts::domain::errors::PartsError;

#[async_trait]
pub trait PartPriceOptionsCreation: Send + Sync + 'static {
    async fn create_part_price_options(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
        part_price_options: Vec<PartPriceOption>,
    ) -> Result<(), PartsError>;
}
