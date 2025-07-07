use crate::quotations::models::dynamodb_requests::BatchDeleteQuotationObject;
use crate::quotations::models::quotation::{Quotation, QuotationStatus};
use crate::shared::{QueryResponse, Result};
use async_trait::async_trait;

#[async_trait]
pub trait QuotationsRepository: Send + Sync + 'static {
    async fn create_quotation(&self, quotation: Quotation) -> Result<()>;
    async fn query_quotations_for_project(
        &self,
        project_id: String,
        page_limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Quotation>, String>>;
    async fn query_quotations_by_status(
        &self,
        status: QuotationStatus,
        page_limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Quotation>, String>>;
    async fn get_quotation_by_id(
        &self,
        project_id: String,
        quotation_id: String,
    ) -> Result<Quotation>;
    async fn update_quotation_status(
        &self,
        project_id: String,
        quotation_id: String,
        status: QuotationStatus,
    ) -> Result<Quotation>;
    /// Delete quotation ONLY if it is not in `PAYED` status.
    async fn try_delete_quotation(&self, project_id: String, quotation_id: String) -> Result<()>;
    async fn batch_delete_parts(&self, data: Vec<BatchDeleteQuotationObject>) -> Result<()>;
}
