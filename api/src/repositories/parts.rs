use crate::parts::models::dynamodb_requests::{BatchDeletePartObject, UpdatablePart};
use crate::parts::models::part::Part;
use crate::shared::{QueryResponse, Result};
use async_trait::async_trait;

#[async_trait]
pub trait PartsRepository: Send + Sync + 'static {
    async fn delete(&self, quotation_id: String, part_id: String) -> Result<Part>;
    async fn get(&self, quotation_id: String, part_id: String) -> Result<Part>;
    async fn query(
        &self,
        quotation_id: String,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Part>, String>>;
    async fn update(&self, updatable_part: UpdatablePart) -> Result<Part>;
    async fn batch_create(&self, parts: Vec<Part>) -> Result<()>;
    async fn batch_delete(&self, data: Vec<BatchDeletePartObject>) -> Result<()>;
    async fn batch_get(&self, quotation_and_part_ids: Vec<(String, String)>) -> Result<Vec<Part>>;
}
