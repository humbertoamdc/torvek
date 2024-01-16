use axum::async_trait;

pub mod create_parts;
pub mod query_parts_for_quotation;

#[async_trait]
pub trait UseCase<Request, Response, Error> {
    async fn execute(&self, request: Request) -> Result<Response, Error>;
}
