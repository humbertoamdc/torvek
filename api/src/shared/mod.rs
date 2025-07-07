use crate::shared::error::Error;
use async_trait::async_trait;

pub mod api_error;
pub mod error;
pub mod extractors;
pub mod file;
pub mod into_error_response;
pub mod money;

pub type Result<T> = std::result::Result<T, Error>;

pub struct QueryResponse<T, V> {
    pub data: T,
    pub cursor: Option<V>,
}

#[async_trait]
pub trait UseCase<Request, Response> {
    async fn execute(&self, request: Request) -> Result<Response>;
}
