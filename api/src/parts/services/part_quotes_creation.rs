use axum::async_trait;

use api_boundary::parts::models::PartQuote;

use crate::parts::domain::errors::PartsError;

#[async_trait]
pub trait PartQuotesCreation: Send + Sync + 'static {
    async fn create_part_quotes_and_update_quotation_status(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
        part_quotes: Vec<PartQuote>,
    ) -> Result<(), PartsError>;
}
