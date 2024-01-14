use crate::parts::domain::errors::PartsError;
use api_boundary::parts::models::Part;
use axum::async_trait;

#[async_trait]
pub trait PartsRepository: Send + Sync + 'static {
    async fn create_parts(&self, parts: Vec<Part>) -> Result<(), PartsError>;
}
