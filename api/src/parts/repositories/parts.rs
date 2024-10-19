use api_boundary::common::error::Error;
use axum::async_trait;

use api_boundary::parts::models::Part;

use crate::parts::domain::updatable_part::UpdatablePart;

#[async_trait]
pub trait PartsRepository: Send + Sync + 'static {
    async fn get_part(&self, quotation_id: String, part_id: String) -> Result<Part, Error>;
    async fn create_parts(&self, parts: Vec<Part>) -> Result<(), Error>;
    async fn query_parts_for_quotation(&self, quotation_id: String) -> Result<Vec<Part>, Error>;
    async fn update_part(&self, updatable_part: UpdatablePart) -> Result<Part, Error>;
}
