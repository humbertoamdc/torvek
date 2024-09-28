use api_boundary::parts::errors::PartsError;
use axum::async_trait;

use api_boundary::parts::models::PartQuote;

#[async_trait]
pub trait PartQuotesCreation: Send + Sync + 'static {
    async fn create_part_quotes_and_update_quotation_status(
        &self,
        project_id: String,
        quotation_id: String,
        part_quotes: Vec<PartQuote>,
    ) -> Result<(), PartsError>;
}
