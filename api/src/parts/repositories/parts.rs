use axum::async_trait;

use api_boundary::parts::models::Part;

use crate::parts::domain::errors::PartsError;
use crate::parts::domain::updatable_part::UpdatablePart;

#[async_trait]
pub trait PartsRepository: Send + Sync + 'static {
    async fn create_parts(&self, parts: Vec<Part>) -> Result<(), PartsError>;
    async fn query_parts_for_quotation(
        &self,
        client_id: String,
        project_id: String,
        quotation_id: String,
    ) -> Result<Vec<Part>, PartsError>;
    async fn update_part(&self, updatable_part: UpdatablePart) -> Result<(), PartsError>;
}
