use axum::async_trait;
use std::collections::HashMap;

use crate::shared::Result;
use api_boundary::parts::models::PartQuote;

#[async_trait]
pub trait PartQuotesCreation: Send + Sync + 'static {
    async fn create_part_quotes_and_update_quotation(
        &self,
        project_id: String,
        quotation_id: String,
        part_quotes_by_part: HashMap<String, Vec<PartQuote>>,
        selected_part_quote_by_part: HashMap<String, String>,
    ) -> Result<()>;
}
