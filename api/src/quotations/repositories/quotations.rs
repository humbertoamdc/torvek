use crate::quotations::domain::errors::QuotationsError;
use api_boundary::quotations::models::{Quotation, QuotationStatus};
use axum::async_trait;

#[async_trait]
pub trait QuotationsRepository: Send + Sync + 'static {
    async fn create_quotation(&self, quotation: Quotation) -> Result<(), QuotationsError>;
    async fn query_quotations_for_project(
        &self,
        client_id: String,
        project_id: String,
    ) -> Result<Vec<Quotation>, QuotationsError>;
    async fn update_quotation_status(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
        status: QuotationStatus,
    ) -> Result<(), QuotationsError>;
}
