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
pub trait UseCase<Input, Response> {
    async fn execute(&self, input: Input) -> Result<Response>;
}

pub type CustomerId = String;
pub type ProjectId = String;
pub type QuoteId = String;
pub type PartId = String;
pub type PartQuoteId = String;
pub type OrderId = String;
pub type FileId = String;
