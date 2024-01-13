use crate::quotations::domain::errors::QuotationsError;
use api_boundary::quotations::models::Quotation;
use axum::async_trait;

#[async_trait]
pub trait QuotationsRepository: Send + Sync + 'static {
    async fn create_quotation(&self, quotation: Quotation) -> Result<(), QuotationsError>;
    async fn query_quotations_for_client(
        &self,
        project_id: String,
    ) -> Result<Vec<Quotation>, QuotationsError>;
}
