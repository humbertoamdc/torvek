use crate::parts::models::part::PartQuote;
use crate::shared::{CustomerId, ProjectId, QuoteId, Result};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait PartQuotesCreation: Send + Sync + 'static {
    async fn create_part_quotes_and_update_quotation(
        &self,
        customer_id: CustomerId,
        project_id: ProjectId,
        quotation_id: QuoteId,
        part_quotes_by_part: HashMap<String, Vec<PartQuote>>,
        selected_part_quote_by_part: HashMap<String, String>,
    ) -> Result<()>;
}
