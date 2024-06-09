use api_boundary::parts::errors::PartsError;
use axum::async_trait;

use api_boundary::parts::models::PartQuote;

#[async_trait]
pub trait PartQuotesRepository: Send + Sync + 'static {
    async fn query_part_quotes_for_part(
        &self,
        part_id: String,
    ) -> Result<Vec<PartQuote>, PartsError>;
}
