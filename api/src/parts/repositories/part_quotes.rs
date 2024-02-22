use axum::async_trait;

use api_boundary::parts::models::PartQuote;

use crate::parts::domain::errors::PartsError;

#[async_trait]
pub trait PartQuotesRepository: Send + Sync + 'static {
    async fn query_part_quotes_for_part(
        &self,
        part_id: String,
    ) -> Result<Vec<PartQuote>, PartsError>;
}
