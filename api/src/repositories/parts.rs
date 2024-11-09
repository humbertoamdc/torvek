use axum::async_trait;

use api_boundary::parts::models::Part;

use crate::parts::domain::dynamodb_requests::{BatchDeletePartObject, UpdatablePart};
use crate::shared::{QueryResponse, Result};

#[async_trait]
pub trait PartsRepository: Send + Sync + 'static {
    async fn get_part(&self, quotation_id: String, part_id: String) -> Result<Part>;
    async fn get_parts_batch(
        &self,
        quotation_and_part_ids: Vec<(String, String)>,
    ) -> Result<Vec<Part>>;
    async fn create_parts(&self, parts: Vec<Part>) -> Result<()>;
    async fn query_parts_for_quotation(
        &self,
        quotation_id: String,
        page_limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Part>, String>>;
    async fn update_part(&self, updatable_part: UpdatablePart) -> Result<Part>;
    async fn delete_part(&self, quotation_id: String, part_id: String) -> Result<Part>;
    async fn batch_delete_parts(&self, data: Vec<BatchDeletePartObject>) -> Result<()>;
}
