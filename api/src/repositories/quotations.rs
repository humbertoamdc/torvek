use crate::quotations::models::dynamodb_requests::BatchDeleteQuotationObject;
use crate::quotations::models::quotation::{Quotation, QuotationStatus};
use crate::shared::{QueryResponse, Result};
use async_trait::async_trait;

pub enum QueryOrderBy {
    ProjectID,
    Status,
}

#[async_trait]
pub trait QuotationsRepository: Send + Sync + 'static {
    async fn create(&self, quotation: Quotation) -> Result<()>;
    async fn query(
        &self,
        project_id: Option<String>,
        status: Option<QuotationStatus>,
        order_by: QueryOrderBy,
        limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Quotation>, String>>;

    async fn get(&self, project_id: String, quotation_id: String) -> Result<Quotation>;
    async fn update(
        &self,
        project_id: String,
        quotation_id: String,
        status: Option<QuotationStatus>,
    ) -> Result<Quotation>;
    /// Delete quotation ONLY if it is not in `PAYED` status.
    async fn delete(&self, project_id: String, quotation_id: String) -> Result<()>;
    async fn batch_delete(&self, data: Vec<BatchDeleteQuotationObject>) -> Result<()>;
}
