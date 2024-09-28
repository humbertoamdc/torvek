use api_boundary::quotations::errors::QuotationsError;
use axum::async_trait;

use api_boundary::quotations::models::{Quotation, QuotationStatus};

#[async_trait]
pub trait QuotationsRepository: Send + Sync + 'static {
    async fn create_quotation(&self, quotation: Quotation) -> Result<(), QuotationsError>;
    async fn query_quotations_for_project(
        &self,
        project_id: String,
    ) -> Result<Vec<Quotation>, QuotationsError>;
    async fn query_quotations_by_status(
        &self,
        status: QuotationStatus,
    ) -> Result<Vec<Quotation>, QuotationsError>;
    async fn get_quotation_by_id(
        &self,
        project_id: String,
        quotation_id: String,
    ) -> Result<Quotation, QuotationsError>;
    async fn update_quotation_status(
        &self,
        project_id: String,
        quotation_id: String,
        status: QuotationStatus,
    ) -> Result<(), QuotationsError>;
}
